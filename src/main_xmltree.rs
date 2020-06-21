use std::fs::File;
use xmltree::Element;
use xmltree::XMLNode;

struct GpxPoint {
    elev: f64,
    lat: f64,
    lon: f64,
}

struct GpxFile {
    sport_type: String,
    points: Vec<GpxPoint>,
}

impl GpxFile {
    pub fn new() -> GpxFile {
        let points = Vec::new();
        GpxFile {
            points: points,
            sport_type: String::from("some"),
        }
    }

    pub fn set_type(&mut self, type_name: &str) {
        self.sport_type = String::from(type_name);
    }

    pub fn add_point(&mut self, point: GpxPoint) {
        self.points.push(point);
    }
}

fn main() {
    let mut gpx_struct = GpxFile::new();
    parse_gpx(&mut gpx_struct);
    write_gpx(&gpx_struct);
}

fn write_gpx(gpx_struct: &GpxFile) {
    let mut root = Element::new("gpx");
    let mut trk = Element::new("trk");
    let mut trkseg =  Element::new("trkseg");
    

    for point in gpx_struct.points.iter() {
        let mut trkpt = Element::new("trkpt");
        
        trkpt.attributes.insert("lat".to_string(), point.lat.to_string());
        trkpt.attributes.insert("lon".to_string(), point.lon.to_string());

        let mut elev = Element::new("ele");
        let elev_text = point.elev.to_string();
        elev.children.push(XMLNode::Text(elev_text));

        trkseg.children.push(XMLNode::Element(trkpt));
    }

   
    trk.children.push(XMLNode::Element(trkseg));
    root.children.push(XMLNode::Element(trk));
    
    write_to_file(&root);
}

fn write_to_file(element: &Element) {
    let output_file = File::create("result.gpx").expect("error creating result.xml");
    element.write(output_file).unwrap();
}


fn parse_gpx(gpx_struct: &mut GpxFile) {
    let file = File::open("activity.gpx").expect("Error opening file activity.gpx");

    let root: Element = Element::parse(file).unwrap();
    let trk = root.get_child("trk").expect("Can't find trk element");
    let type_elem = trk.get_child("type").unwrap().get_text().unwrap();
    gpx_struct.set_type(&type_elem);

    for trk_segment in trk
        .children
        .iter()
        .filter(|x| x.as_element().unwrap().name == "trkseg")
    {
        let trk_segment_element = trk_segment.as_element().unwrap();
        for trk_point in trk_segment_element.children.iter() {
            let point_element = trk_point.as_element().unwrap();
            let attributes = &point_element.attributes;
            let lat = attributes.get("lat").unwrap();
            let lon = attributes.get("lon").unwrap();

            let latf64 = lat.parse::<f64>().unwrap();
            let lonf64 = lon.parse::<f64>().unwrap();

            let elev_elem = point_element.get_child("ele").unwrap();
            let ele = elev_elem.get_text().unwrap();
            let elef64 = ele.parse::<f64>().unwrap();
            let point = GpxPoint {
                lat: latf64,
                lon: lonf64,
                elev: elef64,
            };

            gpx_struct.add_point(point);
        }
    }
}
