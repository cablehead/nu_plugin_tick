use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin::{EngineInterface, EvaluatedCall, Plugin, PluginCommand};
use nu_protocol::{LabeledError, PipelineData, Signature, Type};

pub struct APlugin;

impl Plugin for APlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Tick)]
    }
}

pub struct Tick;

impl PluginCommand for Tick {
    type Plugin = APlugin;
    fn name(&self) -> &str {
        "tick"
    }

    fn usage(&self) -> &str {
        "ticks ticks"
    }

    fn signature(&self) -> Signature {
        Signature::build("from sse").input_output_types(vec![
            (Type::ListStream, Type::ListStream),
            (Type::String, Type::ListStream),
        ])
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        _call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        eprintln!("input: {:?}", &input);
        for x in input {
            eprintln!("X: {:?}", &x);
        }
        Ok(PipelineData::Empty)
    }
}

fn main() {
    serve_plugin(&mut APlugin {}, MsgPackSerializer {})
}
