use tracing_subscriber::EnvFilter;
use uuid::Uuid;

/// Representation of a Patient in our synthetic patient generator
#[derive(Debug)]
struct Patient {
    id: Uuid,
    age: u8,
    sex: String,
    diagnosis_codes: Vec<String>,
}

impl std::fmt::Display for Patient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Patient({}, age {}, {:?})",
            self.id, self.age, self.diagnosis_codes
        )
    }
}

fn main() {
    // Initialize the tracing subscriber with an environment filter to control log levels
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))
        .init();

    // Create a sample patient with random data
    let patient = Patient {
        id: Uuid::new_v4(),
        age: 67,
        sex: "F".to_string(),
        diagnosis_codes: vec!["E11.9".to_string()],
    };

    // Using Display (%)
    tracing::info!(patient = %patient, "writing synthetic patient to reference database");

    // Using Debug (?)
    tracing::info!(patient = ?patient, "writing synthetic patient to reference database");

    // This is the same as the debug statement above!
    tracing::info!(?patient, "writing synthetic patient to reference database");
}
