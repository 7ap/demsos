use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expression {
    r#type: String,
    id: usize,
    color: String,
    latex: String,
    #[serde(rename = "fillOpacity")]
    fill_opacity: String,
    #[serde(rename = "lineOpacity")]
    line_opacity: String,
    #[serde(rename = "lineWidth")]
    line_width: String,
}

pub fn parse_image(image: DynamicImage) -> Vec<Expression> {
    let mut expressions: Vec<Expression> = Vec::new();

    for (x, y, pixel) in image.pixels() {
        let expression = Expression {
            r#type: String::from("expression"),
            id: expressions.len(),
            color: format!("rgb({}, {}, {})", pixel.0[0], pixel.0[1], pixel.0[2]),
            #[rustfmt::skip]
            latex: format!("{}\\le x\\le{}\\left\\{{{}\\le y\\le{}\\right\\}}", x, x + 1, y, y + 1), // FIXME: this makes the image upside down...
            fill_opacity: String::from(""),
            line_opacity: String::from(""),
            line_width: String::from(""),
        };

        expressions.push(expression);
    }

    expressions
}
