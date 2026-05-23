use rand::Rng;
use std::time::Duration;
use tracing_subscriber::fmt::format::FmtSpan;
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

/// Representation of patient demographics
#[derive(Debug)]
struct Demographics {
    age: u8,
    sex: String,
}

/// Generate patient demographics with random age and sex, and
/// a simulated delay to mimic a database query, logging the time
/// taken for the operation. Unlike the previous version, we are
/// now using the elapsed time to log the duration of the operation, which can be useful for performance monitoring in Grafana.
#[tracing::instrument]
fn generate_demographics() -> Demographics {
    let start = std::time::Instant::now();

    let delay = rand::thread_rng().gen_range(10..50);
    std::thread::sleep(Duration::from_millis(delay));

    let age = rand::thread_rng().gen_range(0..100);
    let sex = if rand::thread_rng().gen_bool(0.5) {
        "M".to_string()
    } else {
        "F".to_string()
    };
    let demographics = Demographics { age, sex };
    let elapsed_ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        age = demographics.age,
        sex = %demographics.sex,
        elapsed_ms,
        "generated patient demographics"
    );
    demographics
}

/// Generate patient medical history based on demographics with a simulated delay to mimic a database query
/// and some fake diagnoses. Note that we are using the `skip` attribute to avoid logging the demographics
/// struct itself, and we are also logging the time taken for this operation as well.
#[tracing::instrument(fields(age = demographics.age), skip(demographics))]
fn generate_medical_history(demographics: &Demographics) -> Vec<String> {
    let start = std::time::Instant::now();

    let delay = rand::thread_rng().gen_range(20..100);
    std::thread::sleep(Duration::from_millis(delay));

    let diagnoses = vec!["E11.9".to_string(), "I10".to_string()];
    let elapsed_ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        diagnoses_count = diagnoses.len(),
        elapsed_ms,
        "generated patient medical history"
    );
    diagnoses
}

/// Simulate writing the patient to a database with a random delay and a random
/// chance of failure, logging the time taken for the operation and any errors
/// that occur. We are using the `skip` attribute to avoid logging the entire
/// patient struct.
#[tracing::instrument(skip(patient))]
fn write_to_database(patient: &Patient) -> Result<(), String> {
    let start = std::time::Instant::now();

    let delay = rand::thread_rng().gen_range(50..200);
    std::thread::sleep(Duration::from_millis(delay));

    let elapsed_ms = start.elapsed().as_millis() as u64;

    // Simulate a random failure with a 10% chance
    if rand::thread_rng().gen_bool(0.1) {
        tracing::error!(
            patient_id = %patient.id,
            elapsed_ms,
            error = "connection refused",
            "failed to write patient to database"
        );
        return Err("connection refused".to_string());
    }

    tracing::info!(patient_id = %patient.id, elapsed_ms, "wrote patient to database");
    Ok(())
}

/// Generate a synthetic patient
fn generate_patient() {
    let patient_id = Uuid::new_v4();
    let span = tracing::info_span!("generate_patient", %patient_id);
    let _guard = span.enter();

    let demographics = generate_demographics();
    let medical_history = generate_medical_history(&demographics);

    let patient = Patient {
        id: patient_id,
        age: demographics.age,
        sex: demographics.sex,
        diagnosis_codes: medical_history,
    };

    match write_to_database(&patient) {
        Ok(()) => tracing::info!("patient generation complete"),
        Err(e) => tracing::warn!(error = %e, "patient generation completed with errors"),
    }
}

fn main() {
    // Initialize the tracing subscriber with an environment filter to control log levels
    // and configure it to log when spans are closed. We are also configuring it to write
    // logs in JSON format to a file
    let file_appender = tracing_appender::rolling::daily("logs", "patient-generator.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .json()
        .with_writer(non_blocking)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_span_events(FmtSpan::CLOSE)
        .init();

    tracing::info!("starting synthetic patient generator");

    loop {
        // Generate a synthetic patient in an infinite loop to
        // continuously generate logs that can be monitored in Grafana. In a real application, you would likely have some condition to break out of this loop or run it in a separate thread.
        generate_patient();

        let delay = rand::thread_rng().gen_range(500..2000);
        std::thread::sleep(Duration::from_millis(delay));
    }
}
