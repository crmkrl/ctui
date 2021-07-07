use std::{io , thread, time, time::{SystemTime, UNIX_EPOCH}, convert::TryInto };
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, BorderType, Paragraph, Gauge, List},
    Terminal,
};
use crate::App;
use chrono::prelude::*;
extern crate chrono;

pub fn draw_ui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let size = f.size();

        let main = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [ Constraint::Percentage(100), ]
                .as_ref(),
            )
            .split(f.size());

        let block = Block::default()
            .borders(Borders::NONE)
            .title(" Telemetry Information ")
            .border_type(BorderType::Double)
            .style(Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD));
        f.render_widget(block, main[0]);

        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(f.size());

        /* DATASET */ //*****************************************************
        let num_of_wheels = 4;
        let total_voltage = 13200.0;    //max volt
        let min_volt = 12000.0;         //min voltage to say that it is 0% battery 
        let mut pack_voltage = Vec::new();

        let total_ampere= 5000.0;
        let min_ampere = 3000.0;         //min current to say that it is 0% battery 
        let mut pack_ampere = Vec::new();

        let mut volt_style = vec![Color::Green, Color::Green, Color::Green, Color::Green];
        let mut amp_style = vec![Color::Green, Color::Green, Color::Green, Color::Green];

        let mut tl_cell = Vec::new();
        let mut tr_cell = Vec::new();
        let mut bl_cell = Vec::new();
        let mut br_cell = Vec::new();

        let mut colour_code = Color::Gray;

        for i in 1..5 {
            match i {
                1 => { 
                        pack_voltage.push(map_range((min_volt as f64, total_voltage as f64), (0.0, 100.0), app.tl_pack_voltage.into())); 
                        pack_ampere.push(map_range((min_ampere as f64, total_ampere as f64), (0.0, 100.0), app.tl_pack_ampere.into())); 
                        if !app.tl_cell_voltages.is_empty() {
                            tl_cell = cell_data(app.tl_cell_voltages.clone(), app.counter.clone());
                        }
                    }
                2 => { 
                        pack_voltage.push(map_range((min_volt as f64, total_voltage as f64), (0.0, 100.0), app.tr_pack_voltage.into())); 
                        pack_ampere.push(map_range((min_ampere as f64, total_ampere as f64), (0.0, 100.0), app.tr_pack_ampere.into())); 
                        if !app.tr_cell_voltages.is_empty() {
                            tr_cell = cell_data(app.tr_cell_voltages.clone(), app.counter.clone());
                        }
                    }
                3 => { 
                        pack_voltage.push(map_range((min_volt as f64, total_voltage as f64), (0.0, 100.0), app.bl_pack_voltage.into())); 
                        pack_ampere.push(map_range((min_ampere as f64, total_ampere as f64), (0.0, 100.0), app.bl_pack_ampere.into())); 
                        if !app.bl_cell_voltages.is_empty() {
                            bl_cell = cell_data(app.bl_cell_voltages.clone(), app.counter.clone());
                        }
                    }
                4 => { 
                        pack_voltage.push(map_range((min_volt as f64, total_voltage as f64), (0.0, 100.0), app.br_pack_voltage.into())); 
                        pack_ampere.push(map_range((min_ampere as f64, total_ampere as f64), (0.0, 100.0), app.br_pack_ampere.into())); 
                        if !app.br_cell_voltages.is_empty() {
                            br_cell = cell_data(app.br_cell_voltages.clone(), app.counter.clone());
                        }
                }
                _ => { },
            }
        }

        //Percentage
        for i in 0..pack_voltage.len() {
            if pack_voltage[i] < 20.0 {
                volt_style[i] = Color::Red;
            } else {
                volt_style[i] = Color::Green;  
            }

            if pack_ampere[i] < 20.0 {
                amp_style[i] = Color::Red;
            } else {
                amp_style[i] = Color::Green;  
            }
        }

        let chunk0 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([  Constraint::Percentage(25),     
                        Constraint::Percentage(25), 
                        Constraint::Percentage(25),  
                        Constraint::Percentage(25)].as_ref())
        .margin(1)
        .split(chunks[0]);

        //TL
        let mut index_tl = 0;
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" TL ")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD));
        f.render_widget(block, chunk0[index_tl]);

        let tl_chunk= Layout::default()
            .direction(Direction::Vertical)
            .constraints([  Constraint::Percentage(30),
                            Constraint::Percentage(30),
                            Constraint::Percentage(50)].as_ref())
            .margin(1)
            .split(chunk0[index_tl]);

        let chunk_style = |title| {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(Span::styled(title, Style::default().add_modifier(Modifier::BOLD)))
            };

        //PACK VOLTAGE ************************************************
        let label = format!("{}% | {}mV", pack_voltage[index_tl] as u16, app.tl_pack_voltage);
        let pack_volt = Gauge::default()
            .block(chunk_style(" Pack Voltage: "))
            .gauge_style(Style::default().fg(volt_style[index_tl]).bg(Color::Black).add_modifier(Modifier::ITALIC))
            .percent((pack_voltage[index_tl] as i32).try_into().unwrap())
            .label(label)
            .use_unicode(true);
        f.render_widget(pack_volt, tl_chunk[0]);

        //PACK CURRENT ************************************************
        // let label = format!("{}{}", pack_ampere[index_tl] as u16, "%");
        let label = format!("{} mA", app.tl_pack_ampere as u16);
        let pack_amp = Gauge::default()
            .block(chunk_style(" Pack Current: "))
            .gauge_style(Style::default().fg(amp_style[index_tl]).bg(Color::Black).add_modifier(Modifier::ITALIC))
            .percent((pack_ampere[index_tl] as i32).try_into().unwrap())
            .label(label)
            .use_unicode(true);
        f.render_widget(pack_amp, tl_chunk[1]);

        //CELL VOLTAGES
        let tl_msg = Paragraph::new(tl_cell.clone())
            .style(Style::default())
            .block(chunk_style(" Cell Voltages (mV) "))
            .alignment(Alignment::Left);
        f.render_widget(tl_msg, tl_chunk[2]);

        //TR
        let mut index_tr = 1;
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" TR ")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD));
        f.render_widget(block, chunk0[index_tr]);

        let tr_chunk= Layout::default()
            .direction(Direction::Vertical)
            .constraints([  Constraint::Percentage(30),
                            Constraint::Percentage(30),
                            Constraint::Percentage(50)].as_ref())
            .margin(1)
            .split(chunk0[index_tr]);

        //PACK VOLTAGE ************************************************
        let label = format!("{}% | {}mV", pack_voltage[index_tr] as u16, app.tr_pack_voltage);
        let pack_volt = Gauge::default()
            .block(chunk_style(" Pack Voltage: "))
            .gauge_style(Style::default().fg(volt_style[index_tr]).bg(Color::Black).add_modifier(Modifier::ITALIC))
            .percent((pack_voltage[index_tr] as i32).try_into().unwrap())
            .label(label)
            .use_unicode(true);
        f.render_widget(pack_volt, tr_chunk[0]);

        //PACK CURRENT ************************************************
        // let label = format!("{}{}", pack_ampere[index_tr] as u16, "%");
        let label = format!("{} mA", app.tr_pack_ampere as u16);
        let pack_amp = Gauge::default()
            .block(chunk_style(" Pack Current: "))
            .gauge_style(Style::default().fg(amp_style[index_tr]).bg(Color::Black).add_modifier(Modifier::ITALIC))
            .percent((pack_ampere[index_tr] as i32).try_into().unwrap())
            .label(label)
            .use_unicode(true);
        f.render_widget(pack_amp, tr_chunk[1]);

        //CELL VOLTAGES
        let tr_msg = Paragraph::new(tr_cell.clone())
            .style(Style::default())
            .block(chunk_style(" Cell Voltages (mV) "))
            .alignment(Alignment::Left);
        f.render_widget(tr_msg, tr_chunk[2]);

        //BL
        let mut index_bl = 2;
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" BL ")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD));
        f.render_widget(block, chunk0[index_bl]);

        let bl_chunk= Layout::default()
            .direction(Direction::Vertical)
            .constraints([  Constraint::Percentage(30),
                            Constraint::Percentage(30),
                            Constraint::Percentage(50)].as_ref())
            .margin(1)
            .split(chunk0[index_bl]);

        //PACK VOLTAGE ************************************************
        let label = format!("{}% | {}mV", pack_voltage[index_bl] as u16, app.bl_pack_voltage);
        let pack_volt = Gauge::default()
            .block(chunk_style(" Pack Voltage: "))
            .gauge_style(Style::default().fg(volt_style[index_bl]).bg(Color::Black).add_modifier(Modifier::ITALIC))
            .percent((pack_voltage[index_bl] as i32).try_into().unwrap())
            .label(label)
            .use_unicode(true);
        f.render_widget(pack_volt, bl_chunk[0]);

        //PACK CURRENT ************************************************
        // let label = format!("{}{}", pack_ampere[index_bl] as u16, "%");
        let label = format!("{} mA", app.bl_pack_ampere as u16);
        let pack_amp = Gauge::default()
            .block(chunk_style(" Pack Current: "))
            .gauge_style(Style::default().fg(amp_style[index_bl]).bg(Color::Black).add_modifier(Modifier::ITALIC))
            .percent((pack_ampere[index_bl] as i32).try_into().unwrap())
            .label(label)
            .use_unicode(true);
        f.render_widget(pack_amp, bl_chunk[1]);

        //CELL VOLTAGES
        let bl_msg = Paragraph::new(bl_cell.clone())
            .style(Style::default())
            .block(chunk_style(" Cell Voltages (mV) "))
            .alignment(Alignment::Left);
        f.render_widget(bl_msg, bl_chunk[2]);

        //BR
        let mut index_br = 3;
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" BR ")
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD));
        f.render_widget(block, chunk0[index_br]);

        let br_chunk= Layout::default()
            .direction(Direction::Vertical)
            .constraints([  Constraint::Percentage(30),
                            Constraint::Percentage(30),
                            Constraint::Percentage(50)].as_ref())
            .margin(1)
            .split(chunk0[index_br]);

        //PACK VOLTAGE ************************************************
        let label = format!("{}% | {}mV", pack_voltage[index_br] as u16, app.br_pack_voltage);
        let pack_volt = Gauge::default()
            .block(chunk_style(" Pack Voltage: "))
            .gauge_style(Style::default().fg(volt_style[index_br]).bg(Color::Black).add_modifier(Modifier::ITALIC))
            .percent((pack_voltage[index_br] as i32).try_into().unwrap())
            .label(label)
            .use_unicode(true);
        f.render_widget(pack_volt, br_chunk[0]);

        //PACK CURRENT ************************************************
        // let label = format!("{}{}", pack_ampere[index_br] as u16, "%");
        let label = format!("{} mA", app.br_pack_ampere as u16);
        let pack_amp = Gauge::default()
            .block(chunk_style(" Pack Current: "))
            .gauge_style(Style::default().fg(amp_style[index_br]).bg(Color::Black).add_modifier(Modifier::ITALIC))
            .percent((pack_ampere[index_br] as i32).try_into().unwrap())
            .label(label)
            .use_unicode(true);
        f.render_widget(pack_amp, br_chunk[1]);

        //CELL VOLTAGES
        let tr_msg = Paragraph::new(br_cell.clone())
            .style(Style::default())
            .block(chunk_style(" Cell Voltages (mV) "))
            .alignment(Alignment::Left);
        f.render_widget(tr_msg, br_chunk[2]);
        //end of method
    })
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    let mut value = to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0);
    if value < 0.0 { value = 0.0 } else if value > 100.0 { value = 100.0; }
    value
}

fn cell_data<'a>(cell_voltage: Vec<u32>, count: u32) -> Vec<Spans<'a>> {
    let mut cell = Vec::new();
    let mut colour_code = Color::Gray;
    let low_cell_voltage = 1500;        //min cell voltage to say that it is low voltage 

    for j in 0..cell_voltage.len() {
        if cell_voltage[j] < low_cell_voltage {
            if count % 2 == 0 {
                colour_code = Color::Red; 
            } else {
                colour_code = Color::Black; 
            }
            
        } else {
            colour_code = Color::Gray;   
        }
        cell.push(Spans::from(Span::styled(format!(" Cell{}: {}", j+1, cell_voltage[j]), 
        Style::default().fg(colour_code).add_modifier(Modifier::ITALIC))));   
    }

    cell
}

pub fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}