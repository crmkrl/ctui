#[derive(Clone)]
pub struct App<'a> {
    pub title: &'a str,
    pub do_nothing: bool,

    pub tl_pack_voltage: u32,
    pub tl_pack_ampere: i32,
    pub tl_cell_voltages: Vec<u32>,

    pub tr_pack_voltage: u32,
    pub tr_pack_ampere: i32,
    pub tr_cell_voltages: Vec<u32>,

    pub bl_pack_voltage: u32,
    pub bl_pack_ampere: i32,
    pub bl_cell_voltages: Vec<u32>,

    pub br_pack_voltage: u32,
    pub br_pack_ampere: i32,
    pub br_cell_voltages: Vec<u32>,

    pub counter: u32,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            do_nothing: false,

            tl_pack_voltage: 14000,
            tl_pack_ampere: 3050,
            tl_cell_voltages: [1000, 1200, 1300, 1400, 5000, 1000, 5000, 4000].to_vec(),

            tr_pack_voltage: 13000,
            tr_pack_ampere: 4000,
            tr_cell_voltages: Vec::new(),

            bl_pack_voltage: 1000,
            bl_pack_ampere: 3500,
            bl_cell_voltages: [1000, 1200, 1300, 1400, 5000, 1000].to_vec(),

            br_pack_voltage: 0,
            br_pack_ampere: 0,
            br_cell_voltages: [1000, 1200, 500, 3000].to_vec(),

            counter: 0,
        }
    }

    pub fn update(&mut self, node_name: &str, pack_voltage: u32, pack_ampere: i32, cell_voltages: Vec<u32>) {
        match node_name {
            "TL" => { self.tl_pack_voltage = pack_voltage; self.tl_pack_ampere = pack_ampere; self.tl_cell_voltages = cell_voltages; } 
            "TR" => { self.tr_pack_voltage = pack_voltage; self.tr_pack_ampere = pack_ampere; self.tr_cell_voltages = cell_voltages; } 
            "BL" => { self.bl_pack_voltage = pack_voltage; self.bl_pack_ampere = pack_ampere; self.bl_cell_voltages = cell_voltages; } 
            "BR" => { self.br_pack_voltage = pack_voltage; self.br_pack_ampere = pack_ampere; self.br_cell_voltages = cell_voltages; } 
            _ => { },
        }
    }

    pub fn start(&mut self) {
        self.counter += 1;
    }
}