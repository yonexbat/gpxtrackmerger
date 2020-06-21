mod xmltreesample;
mod xmlrssample;

fn main() {

    // parse and write gpx track with StaX (xmlrs)
    xmlrssample::sample_xml_rs();

    // parse and write gpx track with DOM (xmltree)
    xmltreesample::sample_xml_tree();
}
