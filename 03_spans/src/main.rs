use rand::Rng;
use std::time::Duration;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;
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

/// Representation of patient demographics
#[derive(Debug)]
struct Demographics {
    age: u8,
    sex: String,
}

/// Generate patient demographics with random age and sex
#[tracing::instrument]
fn generate_demographics() -> Demographics {
    let age = rand::thread_rng().gen_range(0..100);
    let sex = if rand::thread_rng().gen_bool(0.5) {
        "M".to_string()
    } else {
        "F".to_string()
    };
    let demographics = Demographics { age, sex };
    tracing::info!(age = demographics.age, sex = %demographics.sex, "generated patient demographics");
    demographics
}

/// Generate patient medical history based on demographics with a simulated delay to mimic a database query
/// and some fake diagnoses
#[tracing::instrument(fields(age = demographics.age), skip(demographics))]
fn generate_medical_history(demographics: &Demographics) -> Vec<String> {
    // Sample from a json file of possible diagnoses based on age but just pretend for now
    // with delay
    std::thread::sleep(Duration::from_millis(100));
    let diagnoses = vec!["E11.9".to_string(), "I10".to_string()];
    tracing::info!(
        diagnoses_count = diagnoses.len(),
        "generated patient medical history"
    );
    diagnoses
}

/// Generate a synthetic patient by creating a span for the entire process and logging relevant information at each step
fn generate_patient() {
    let patient_id = Uuid::new_v4();
    let span = tracing::info_span!("generate_patient", %patient_id);
    let _guard = span.enter();

    let demographics = generate_demographics();
    let _ = generate_medical_history(&demographics);
    tracing::info!("writing synthetic patient to reference database");
}

fn main() {
    // Initialize the tracing subscriber with an environment filter to control log levels and configure it to log when spans are closed
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // Generate one synthetic patient to see the spans and logs in action
    generate_patient();
}
