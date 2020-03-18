use cairo::{Format, ImageSurface, Surface};
use std::fmt;
use std::fs::File;

use crate::loader::SourceLoader;
use crate::node::Root;
use crate::renderer::render;
use crate::{Context, ContextImpl};

#[derive(Debug)]
pub enum CompileError {
    LoadError(String),
    ParseError(String),
    OutputError(String),
    ExportError,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompileError::LoadError(x) => f.write_str(x),
            CompileError::ParseError(x) => f.write_str(x),
            CompileError::OutputError(x) => {
                f.write_fmt(format_args!("file output error! path: '{}'", x))
            }
            CompileError::ExportError => f.write_str("file export error!"),
        }
    }
}

pub struct Compiler {}

impl Compiler {
    pub fn compile_to_png(src_path: String, out_path: String) -> Result<u8, CompileError> {
        let src = SourceLoader::load(src_path.as_str())
            .map_err(|x| -> CompileError { CompileError::LoadError(x.to_string()) })?;
        let node = Root::parse(&src)
            .map_err(|x| -> CompileError { CompileError::ParseError(x.to_string()) })?;
        let surface = ImageSurface::create(Format::ARgb32, node.width, node.height).unwrap();

        Self::render_context(&surface, node)?;

        let mut f = File::create(out_path.as_str())
            .map_err(|_| -> CompileError { CompileError::OutputError(out_path) })?;
        surface
            .write_to_png(&mut f)
            .map_err(|_| -> CompileError { CompileError::ExportError })?;
        surface.finish();
        Ok(0)
    }

    fn render_context(surface: &Surface, node: Root) -> Result<u8, CompileError> {
        let mut context = ContextImpl::new(&surface);
        context.save();
        context.set_source_rgba(
            node.color.r as f64 / 255.0,
            node.color.g as f64 / 255.0,
            node.color.b as f64 / 255.0,
            node.color.a as f64,
        );
        context.rectangle(0.0, 0.0, node.width as f64, node.height as f64);
        context.fill();
        context.restore();
        for layer in node.layers {
            render(&mut context, layer);
        }
        surface.flush();
        Ok(0)
    }
}
