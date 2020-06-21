pub fn sample_xml_rs() {
    let mut transformer = Transformer::new();
    let output_file_name = "output.gpx";
    let input_file_names = vec!["activity.gpx"];
    transformer
        .process(&output_file_name, &input_file_names)
        .expect("error");
}

extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;
use xml::writer::EmitterConfig;

pub struct Transformer {
    trkpoint_in_progress: bool,
}

impl Transformer {
    pub fn new() -> Transformer {
        Transformer {
            trkpoint_in_progress: false,
        }
    }

    pub fn process(
        &mut self,
        output_file_name: &str,
        input_file_names: &Vec<&str>,
    ) -> Result<(), &'static str> {
        // create xml writer
        let mut xml_writer = self.create_output_file(&output_file_name);

        // write header.
        self.create_output_file_header(&mut xml_writer)?;

        // iterate over input files
        for input_file_name in input_file_names.iter() {
            self.read_input_file(&mut xml_writer, input_file_name)?;
        }

        // write footer
        self.create_output_file_footer(&mut xml_writer)?;

        Ok(())
    }

    fn create_output_file(&mut self, output_file_name: &str) -> xml::EventWriter<std::fs::File> {
        // create xml writer
        let output_file = File::create(output_file_name).unwrap();
        let xml_writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(output_file);
        xml_writer
    }

    fn read_input_file(
        &mut self,
        xml_writer: &mut xml::EventWriter<std::fs::File>,
        input_file_name: &str,
    ) -> Result<(), &'static str> {
        let input_file: File =
            File::open(input_file_name).expect("Error opening file activity.gpx");
        let input_file = BufReader::new(input_file);
        let parser = EventReader::new(input_file);
        for e in parser {
            match e {
                // start elem trkpt
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) if (&name.local_name == "trkpt") => {
                    self.trkpoint_in_progress = true;
                    self.element_start(xml_writer, &attributes, &name, &namespace)?;
                }

                // end elem trkpt
                Ok(xml::reader::XmlEvent::EndElement { name, .. })
                    if (&name.local_name == "trkpt") =>
                {
                    self.trkpoint_in_progress = false;
                    self.element_end(xml_writer);
                }

                // start any element
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) if (self.trkpoint_in_progress == true) => {
                    self.element_start(xml_writer, &attributes, &name, &namespace)?;
                }

                // end any element
                Ok(xml::reader::XmlEvent::EndElement { .. })
                    if (self.trkpoint_in_progress == true) =>
                {
                    self.element_end(xml_writer);
                }

                // characters
                Ok(xml::reader::XmlEvent::Characters(characters))
                    if (self.trkpoint_in_progress == true) =>
                {
                    self.characters(xml_writer, &characters)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn create_output_file_header(
        &mut self,
        xml_writer: &mut xml::EventWriter<std::fs::File>,
    ) -> Result<(), &'static str> {
        // gpx
        let event: xml::writer::XmlEvent = xml::writer::XmlEvent::start_element("gpx").into();
        xml_writer.write(event).expect("error writing to file");

        // trk
        let event: xml::writer::XmlEvent = xml::writer::XmlEvent::start_element("trk").into();
        xml_writer.write(event).expect("error writing to file");

        // trkseg
        let event: xml::writer::XmlEvent = xml::writer::XmlEvent::start_element("trkseg").into();
        xml_writer.write(event).expect("error writing to file");
        Ok(())
    }

    pub fn create_output_file_footer(
        &mut self,
        xml_writer: &mut xml::EventWriter<std::fs::File>,
    ) -> Result<(), &'static str> {
        for _x in 1..3 {
            self.write_closing_tag(xml_writer)?;
        }

        Ok(())
    }

    pub fn write_closing_tag(
        &mut self,
        xml_writer: &mut xml::EventWriter<std::fs::File>,
    ) -> Result<(), &'static str> {
        let event: xml::writer::XmlEvent = xml::writer::XmlEvent::end_element().into();
        xml_writer.write(event).expect("error writing closing tags");

        Ok(())
    }

    pub fn characters(
        &mut self,
        xml_writer: &mut xml::EventWriter<std::fs::File>,
        text: &str,
    ) -> Result<(), &'static str> {
        let event: xml::writer::XmlEvent = xml::writer::XmlEvent::characters(text).into();
        xml_writer.write(event).expect("error writing closing tags");
        Ok(())
    }

    fn element_start(
        &mut self,
        xml_writer: &mut xml::EventWriter<std::fs::File>,
        atttributes: &Vec<OwnedAttribute>,
        name: &xml::name::OwnedName,
        _namespace: &xml::namespace::Namespace,
    ) -> Result<(), &'static str> {
        let name = &name.local_name[..];
        let mut builder: xml::writer::events::StartElementBuilder<'_> =
            xml::writer::XmlEvent::start_element(name);

        for attr in atttributes.iter() {
            let attribute_name = &attr.name.local_name[..];
            let attribute_value = &attr.value[..];
            builder = builder.attr(attribute_name, attribute_value);
        }
        let write_event: xml::writer::XmlEvent = builder.into();
        xml_writer.write(write_event).expect("error writing");
        Ok(())
    }

    fn element_end(&mut self, xml_writer: &mut xml::EventWriter<std::fs::File>) {
        let end_event: xml::writer::XmlEvent = xml::writer::XmlEvent::end_element().into();
        xml_writer
            .write(end_event)
            .expect("error writing closing tags");
    }
}
