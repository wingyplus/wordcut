use rustler::{Env, Error, ResourceArc, Term};
use wordcut_engine::Wordcut;

struct WordcutResource {
    wordcut: Wordcut,
}

impl WordcutResource {
    fn new(path: &std::path::Path) -> std::io::Result<Self> {
        let dict = wordcut_engine::load_dict(path)?;
        Ok(Self {
            wordcut: Wordcut::new(dict),
        })
    }
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(WordcutResource, env);
    true
}

#[rustler::nif]
fn new(path: String) -> Result<ResourceArc<WordcutResource>, Error> {
    match WordcutResource::new(std::path::Path::new(&path)) {
        Ok(resource) => Ok(ResourceArc::new(resource)),
        Err(_) => Err(Error::Atom("load_dict_error")),
    }
}

#[rustler::nif]
fn segment(resource: ResourceArc<WordcutResource>, text: &str) -> Vec<String> {
    resource.wordcut.segment_into_strings(text)
}

rustler::init!(
    "Elixir.Wordcut.Engine.WordcutEngine",
    [new, segment],
    load = load
);
