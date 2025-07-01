use super::SoapBinding;
use crate::{
    error::WriterResult,
    model::{
        Namespace,
        field::as_field_name,
        helpers::{write_check_restrictions_footer, write_check_restrictions_header},
    },
    reader::WriteXml,
};
use inflector::cases::{pascalcase::to_pascal_case, snakecase::to_snake_case};
use reqwest::Url;
use std::{io, rc::Rc};

impl<W> WriteXml<W> for SoapBinding
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        for (operation_name, operation) in &self.operations {
            writeln!(writer, "\n/* {operation_name} */\n")?;

            // input
            let operation_name = to_pascal_case(operation_name);
            let envelope_name = format!("{operation_name}InputEnvelope");
            let soap_operation = &operation.input;
            write_soap_operation(writer, &envelope_name, soap_operation, &self.target_namespaces)?;

            // output
            if let Some(output) = &operation.output {
                let envelope_name = format!("{operation_name}OutputEnvelope");
                write_soap_operation(writer, &envelope_name, output, &self.target_namespaces)?;
            }

            // action
            if let Some(action) = &operation.action {
                write_soap_action(writer, &operation_name, operation, action)?;
            }
        }

        Ok(())
    }
}

fn write_soap_action<W>(
    writer: &mut W,
    operation_name: &str,
    operation: &super::SoapOperation,
    action: &Url,
) -> WriterResult<()>
where
    W: io::Write,
{
    // generate an async fn for the operation
    let rust_fn_name = to_snake_case(operation_name);
    let request_name = format!("{operation_name}InputEnvelope");
    let response_name = operation
        .output
        .as_ref()
        .map(|_| format!("{operation_name}OutputEnvelope"));

    if let Some(res_name) = response_name {
        writeln!(
            writer,
            "pub async fn {rust_fn_name}(req: {request_name}, credentials: Option<(String, String)>) -> error::SoapResult<{res_name}> {{"
        )?;
    } else {
        writeln!(
            writer,
            "pub async fn {rust_fn_name}(req: {request_name}, credentials: Option<(String, String)>) -> error::SoapResult<()> {{"
        )?;
    }

    writeln!(writer, "    let url = \"{action}\";")?;
    writeln!(writer, "    helpers::send_soap_request(url, credentials, req).await")?;
    writeln!(writer, "}}")?;

    Ok(())
}

fn write_soap_operation<W>(
    writer: &mut W,
    envelope_name: &str,
    soap_operation: &super::SoapEnvelope,
    target_namespaces: &[Rc<Namespace>],
) -> WriterResult<()>
where
    W: io::Write,
{
    // build the list of xmlns namespaces to include in the envelope
    // start with: soapenv="http://schemas.xmlsoap.org/soap/envelope/"
    let mut xmlns = vec![("soapenv", "http://schemas.xmlsoap.org/soap/envelope/")];
    for namespace in target_namespaces {
        xmlns.push((namespace.abbreviation.as_str(), namespace.namespace.as_str()));
    }
    let namespaces = xmlns
        .iter()
        .map(|(k, v)| format!("\"{k}\" = \"{v}\""))
        .collect::<Vec<String>>()
        .join(", ");

    // yaserde namespaces header
    let yaserde_ns_header = format!("namespaces = {{ {namespaces} }}");

    if !soap_operation.headers.is_empty() {
        let rust_name = format!("{envelope_name}Header");
        writeln!(writer, "#[derive(Debug, Default, YaSerialize, YaDeserialize)]")?;
        writeln!(writer, "#[yaserde(prefix = \"soapenv\", {yaserde_ns_header})]")?;
        writeln!(writer, "pub struct {rust_name} {{")?;
        for (part_name, header) in &soap_operation.headers {
            let field_name = as_field_name(part_name);
            let rust_type = header.rust_type.xml_name().expect("xml_name not found");

            if let Some(namespace) = header.in_namespace.as_ref() {
                let abbreviation = namespace.abbreviation.as_str();
                writeln!(
                    writer,
                    "#[yaserde(prefix = \"{abbreviation}\", rename = \"{part_name}\")]"
                )?;
            } else {
                writeln!(writer, "    #[yaserde(rename = \"{part_name}\")]")?;
            }

            // todo: we should check if the "mustUnderstand" == 1 to make the field required
            if let Some(namespace) = header.in_namespace.as_ref() {
                let mod_name = namespace.rust_mod_name.as_str();
                writeln!(writer, "    pub {field_name}: Option<{mod_name}::{rust_type}>,",)?;
            } else {
                writeln!(writer, "    pub {field_name}: Option<{rust_type}>",)?;
            }
        }
        writeln!(writer, "}}")?;

        // Write the restriction check
        write_check_restrictions_header(writer, &rust_name, None)?;
        for (part_name, _header) in &soap_operation.headers {
            let field_name = as_field_name(part_name);
            writeln!(
                writer,
                "     self.{field_name}.check_restrictions(restrictions.clone())?;"
            )?;
        }
        writeln!(writer, "    Ok(())")?;
        write_check_restrictions_footer(writer)?;
    }

    let body = soap_operation.body.rust_type.xml_name().expect("xml_name not found");
    let body_rust_name = to_pascal_case(body);
    let body_field_name = as_field_name(&to_snake_case(body));
    let xml_name = soap_operation.body.rust_type.xml_name().expect("xml_name not found");

    writeln!(writer, "#[derive(Debug, Default, YaSerialize, YaDeserialize)]")?;

    if let Some(namespace) = soap_operation.body.in_namespace.as_ref() {
        let abbreviation = namespace.abbreviation.as_str();
        writeln!(writer, "#[yaserde(prefix = \"{abbreviation}\", {yaserde_ns_header})]")?;
    } else {
        writeln!(writer, "#[yaserde(rename = \"Envelope\", {yaserde_ns_header})]")?;
    }

    let rust_name = format!("{envelope_name}Body");
    writeln!(writer, "pub struct {rust_name} {{")?;
    if let Some(namespace) = soap_operation.body.in_namespace.as_ref() {
        let mod_name = namespace.rust_mod_name.as_str();
        let abbreviation = namespace.abbreviation.as_str();
        writeln!(
            writer,
            "    #[yaserde(prefix = \"{abbreviation}\", rename = \"{xml_name}\")]"
        )?;
        writeln!(writer, "    pub {body_field_name}: {mod_name}::{body_rust_name},",)?;
    } else {
        writeln!(writer, "    #[yaserde(rename = \"{xml_name}\")]")?;
        writeln!(writer, "    pub {body_field_name}: {body_rust_name},")?;
    }
    writeln!(writer, "}}")?;

    write_check_restrictions_header(writer, &rust_name, None)?;
    writeln!(writer, "     self.{body_field_name}.check_restrictions(restrictions)")?;
    write_check_restrictions_footer(writer)?;

    writeln!(writer, "#[derive(Debug, Default, YaSerialize, YaDeserialize)]")?;
    writeln!(
        writer,
        "#[yaserde(prefix = \"soapenv\", rename = \"Envelope\", {yaserde_ns_header})]"
    )?;
    writeln!(writer, "pub struct {envelope_name} {{")?;

    if !soap_operation.headers.is_empty() {
        writeln!(writer, "    #[yaserde(prefix = \"soapenv\", rename = \"Header\")]")?;
        writeln!(writer, "    pub header: {envelope_name}Header,")?;
    }

    writeln!(writer, "    #[yaserde(prefix = \"soapenv\", rename = \"Body\")]")?;
    writeln!(writer, "    pub body: {envelope_name}Body,",)?;
    writeln!(writer, "}}")?;

    write_check_restrictions_header(writer, envelope_name, None)?;
    if !soap_operation.headers.is_empty() {
        writeln!(writer, "     self.header.check_restrictions(restrictions.clone())?;")?;
    }
    writeln!(writer, "     self.body.check_restrictions(restrictions)")?;
    write_check_restrictions_footer(writer)?;

    Ok(())
}
