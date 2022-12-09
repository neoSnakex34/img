use image::{RgbaImage, Pixels};
use crate::cell::{Cell, MazeError};
pub struct Maze{

    cells: Vec<Vec<Cell>>,
    height: usize, 
    width: usize,
    start: (usize, usize),
    end: (usize, usize)

}

impl TryFrom<RgbaImage> for Maze{

    type Error = MazeError;

    fn try_from(img: RgbaImage) -> Result<Self, MazeError>{
        if !img.enumerate_pixels().all(|(_,_,p)| Cell::try_from(*p).is_ok()){
            return Err(MazeError::InvalidPixelColor)
        }
        
        img.enumerate_rows().for_each(|(_, pixels)|{
            // in questo scope ho tutta la riga, si è detto di mettere ogni riga in un vettore
            // facente parte di cell?
            pixels.for_each(|(_,_,p|{
                println!("{:?}", p);
            } )
        });
        //TODO controllo ci sia un pixel rosso e uno verde e li prendo (in start e end)
        //TODO costruisci cells (enumerate_rows)

        Ok(Self{cells, height: img.height() as usize, width: img.width() as usize, start, end})

    }

    

}

impl Maze{

    pub fn solve(&mut self){

    } 

    pub fn print_over_image(&self, img: &mut RgbaImage){
        
    } 

}
