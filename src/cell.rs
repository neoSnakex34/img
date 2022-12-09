use image::Rgba;


pub enum Cell{
    Wall,
    Path,
    WalkedPath, 
    Start,
    End  
}

impl TryFrom<Rgba<u8>> for Cell{
    type Error = MazeError;

    fn try_from(value: Rgba<u8>) -> Result<Self, Self::Error> {
        match value.0 {
            [0,0,0,255] => Ok(Self::Wall),
            [255,255,255,255] => Ok(Self::Path),
            [255,0,0,255] => Ok(Self::Start),
            [0,255,0,255] => Ok(Self::End),
            _ => Err(MazeError::InvalidPixelColor)
        }
    }
}

#[derive(Debug)]
pub enum MazeError {
    InvalidPixelColor,
    IncompatibleDimensons,
    MissingEndpoints,
    TooManyEndpoints,
}

impl std::fmt::Display for MazeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidPixelColor => writeln!(f, "There are invalid colors inside the image."),
            Self::IncompatibleDimensons => writeln!(f, "The image has incompatible declared dimensions."),
            Self::MissingEndpoints => writeln!(f, "There are no endpoints (start and end)."),
            Self::TooManyEndpoints => writeln!(f, "There are too many endpoints (start and end)."),
        }
    }
}

impl std::error::Error for MazeError {}

