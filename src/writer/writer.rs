use std::io::{Write, BufWriter};

use {Error, Entry, Property};

/// The font writer.
pub struct Writer<T: Write> {
	stream: BufWriter<T>,
}

impl<T: Write> From<T> for Writer<T> {
	fn from(stream: T) -> Writer<T> {
		Writer {
			stream: BufWriter::new(stream),
		}
	}
}

impl<T: Write> Writer<T> {
	/// Write an entry.
	pub fn entry(&mut self, entry: &Entry) -> Result<(), Error> {
		match entry {
			&Entry::StartFont(ref string) =>
				try!(self.stream.write_all(format!("STARTFONT {}\n", string).as_bytes())),

			&Entry::Comment(ref string) =>
				try!(self.stream.write_all(format!("COMMENT \"{}\"\n", string).as_bytes())),

			&Entry::ContentVersion(ref string) =>
				try!(self.stream.write_all(format!("CONTENTVERSION {}\n", string).as_bytes())),

			&Entry::Font(ref string) =>
				try!(self.stream.write_all(format!("FONT {}\n", string).as_bytes())),

			&Entry::Size(pt, x, y) =>
				try!(self.stream.write_all(format!("SIZE {} {} {}\n", pt, x, y).as_bytes())),

			&Entry::Chars(chars) =>
				try!(self.stream.write_all(format!("CHARS {}\n", chars).as_bytes())),

			&Entry::FontBoundingBox(ref bbx) =>
				try!(self.stream.write_all(format!("FONTBOUNDINGBOX {} {} {} {}\n",
					bbx.width, bbx.height, bbx.x, bbx.y).as_bytes())),

			&Entry::EndFont =>
				try!(self.stream.write_all("ENDFONT\n".as_bytes())),

			&Entry::StartProperties(len) =>
				try!(self.stream.write_all(format!("STARTPROPERTIES {}\n", len).as_bytes())),

			&Entry::Property(ref name, ref value) =>
				match value {
					&Property::String(ref string) =>
						try!(self.stream.write_all(format!("{} \"{}\"\n", name, string).as_bytes())),

					&Property::Integer(value) =>
						try!(self.stream.write_all(format!("{} {}\n", name, value).as_bytes())),
				},

			&Entry::EndProperties =>
				try!(self.stream.write_all("ENDPROPERTIES\n".as_bytes())),

			&Entry::StartChar(ref name) =>
				try!(self.stream.write_all(format!("STARTCHAR {}\n", name).as_bytes())),

			&Entry::Encoding(value) =>
				try!(self.stream.write_all(format!("ENCODING {}\n", value as u32).as_bytes())),

			&Entry::ScalableWidth(x, y) =>
				try!(self.stream.write_all(format!("SWIDTH {} {}\n", x, y).as_bytes())),

			&Entry::DeviceWidth(x, y) =>
				try!(self.stream.write_all(format!("DWIDTH {} {}\n", x, y).as_bytes())),

			&Entry::BoundingBox(ref bbx) =>
				try!(self.stream.write_all(format!("BBX {} {} {} {}\n",
					bbx.width, bbx.height, bbx.x, bbx.y).as_bytes())),

			&Entry::Bitmap(ref map) => {
				try!(self.stream.write_all("BITMAP\n".as_bytes()));

				for y in 0 .. map.height() {
					let mut value: usize = 0;

					for x in 0 .. map.width() {
						value <<= 1;
						value  |= if map.get(x, y) { 1 } else { 0 };
					}

					try!(self.stream.write_all(format!("{:02X}\n", value).as_bytes()));
				}
			},

			&Entry::EndChar =>
				try!(self.stream.write_all("ENDCHAR\n".as_bytes())),

			&Entry::Unknown(..) =>
				unreachable!(),
		}

		Ok(())
	}
}