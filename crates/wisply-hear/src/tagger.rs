use std::path::Path;

use ort::environment::Environment;
use ort::session::Session;
use ort::value::Tensor;

pub struct Tagger {
    session: Session,
    pub classes: usize,
}

fn open(runtime: &Path, path: &Path, threads: usize) -> Result<Session, String> {
    let environment: Environment = ort::init_from(runtime)
        .map_err(|e| format!("{}: {e}", runtime.display()))?
        .build()
        .map_err(|e| e.to_string())?;
    let session = start(&environment, path, threads).map_err(|e| e.to_string())?;
    Ok(session)
}

fn start(environment: &Environment, path: &Path, threads: usize) -> ort::Result<Session> {
    let builder = Session::builder(environment)?;
    let mut builder = builder.with_intra_threads(threads).map_err(ort::Error::<()>::from)?;
    builder.commit_from_file(path)
}

impl Tagger {
    pub fn load(runtime: &Path, path: &Path, threads: usize, classes: usize) -> Result<Tagger, String> {
        let session = open(runtime, path, threads)?;
        Ok(Tagger { session, classes })
    }

    pub fn run(&mut self, bands: usize, frames: usize, feats: Vec<f32>) -> Result<Vec<f32>, String> {
        let input = Tensor::from_array(([1usize, bands, frames], feats)).map_err(|e| e.to_string())?;
        let outputs = self.session.run(ort::inputs![input]).map_err(|e| e.to_string())?;
        let (_, data) = outputs[0].try_extract_tensor::<f32>().map_err(|e| e.to_string())?;
        if data.len() != self.classes {
            return Err(format!("{} scores, {} labels", data.len(), self.classes));
        }
        Ok(data.to_vec())
    }
}
