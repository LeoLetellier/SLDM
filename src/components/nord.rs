use hex_color::{HexColor, ParseHexColorError};

trait ThemeColor {
    fn get_hex_code(&self) -> &str;
    fn get_color(&self) -> Result<HexColor, ParseHexColorError>;
}

trait ThemePalette {
    fn get_hex_codes(&self) -> Vec<impl ThemeColor>;
    fn get_colors(&self) -> Vec<Result<HexColor, ParseHexColorError>>;
}

#[derive(Debug)]
enum NordColors {
    // Polar Night
    NORD0,
    NORD1,
    NORD2,
    NORD3,

    // Snow Storm
    NORD4,
    NORD5,
    NORD6,

    // Frost
    NORD7,
    NORD8,
    NORD9,
    NORD10,

    // Aurora
    NORD11,
    NORD12,
    NORD13,
    NORD14,
    NORD15,
}

impl ThemeColor for NordColors {
    fn get_hex_code(&self) -> &str {
        match self {
            NordColors::NORD0 => "#2e3440",
            NordColors::NORD1 => "#3b4252",
            NordColors::NORD2 => "#434c5e",
            NordColors::NORD3 => "#4c566a",

            NordColors::NORD4 => "#d8dee9",
            NordColors::NORD5 => "#e5e9f0",
            NordColors::NORD6 => "#eceff4",

            NordColors::NORD7 => "#8fbcbb",
            NordColors::NORD8 => "#88c0d0",
            NordColors::NORD9 => "#81a1c1",
            NordColors::NORD10 => "#5e81ac",

            NordColors::NORD11 => "#bf616a",
            NordColors::NORD12 => "#d08770",
            NordColors::NORD13 => "#ebcb8b",
            NordColors::NORD14 => "#a3be8c",
            NordColors::NORD15 => "#b48ead",
        }
    }

    fn get_color(&self) -> Result<HexColor, ParseHexColorError> {
        HexColor::parse(self.get_hex_code())
    }
}

#[derive(Debug)]
enum NordPalette {
    POLAR_NIGHT,
    SNOW_STORM,
    FROST,
    AURORA,
}

impl ThemePalette for NordPalette {
    fn get_hex_codes(&self) -> Vec<impl ThemeColor> {
        match self {
            NordPalette::POLAR_NIGHT => vec![NordColors::NORD0, NordColors::NORD1, NordColors::NORD2, NordColors::NORD3],
            NordPalette::SNOW_STORM => vec![NordColors::NORD4, NordColors::NORD5, NordColors::NORD6],
            NordPalette::FROST => vec![NordColors::NORD7, NordColors::NORD8, NordColors::NORD9, NordColors::NORD10],
            NordPalette::AURORA => vec![NordColors::NORD11, NordColors::NORD12, NordColors::NORD13, NordColors::NORD14, NordColors::NORD15],
        }
    }

    fn get_colors(&self) -> Vec<Result<HexColor, ParseHexColorError>> {
        self.get_hex_codes().iter().map(|code| HexColor::parse(code.get_hex_code())).collect()
    }
}