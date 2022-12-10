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
        // controllo che tutti i pixel siano validi 
        if !img.enumerate_pixels().all(|(_,_,p)| Cell::try_from(*p).is_ok()){
            return Err(MazeError::InvalidPixelColor)
        }

        let mut start = None;
        let mut end = None;

        // let mut start_flag = false;
        // let mut end_flag = false;
        for (_,row) in img.enumerate_rows(){
            for (x,y,p) in row{
                match Cell::try_from(*p).unwrap() {
                    
                    Cell::Start => {
                        if start.is_some(){
                            return Err(MazeError::TooManyEndpoints)
                        }else{
                            start = Some((x as usize, y as usize));
                        }
                    }

                    Cell::End =>{
                        if end.is_some(){
                            return Err(MazeError::TooManyEndpoints)
                        }else{
                            end = Some((x as usize, y as usize));
                        }
                    }

                    _ => {}
                }
            }
        }

        if start.is_none() || end.is_none() {
            return  Err(MazeError::MissingEndpoints);
        }

    
        //TODO costruisci cells (enumerate_rows)
        let cells = img.enumerate_rows()
                                                .map(|(_, row)| {
                                                    row.into_iter().map(|(_,_,p)| Cell::try_from(*p).unwrap()).collect()
                                                }).collect();

        Ok(Self{cells, height: img.height() as usize, width: img.width() as usize, start: start.unwrap(), end: end.unwrap()})

    }

    

}

impl Maze{

    pub fn solve(&mut self){

    } 

    pub fn print_over_image(&self, img: &mut RgbaImage){
        
    } 

}
