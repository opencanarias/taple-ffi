use taple_core::{Notification};

pub enum TapleNotification {
    /// A new subject has been generated
    NewSubject {
        subject_id: String,
    },
    /// A new event has been generated
    NewEvent {
        sn: u64,
        subject_id: String,
    },
    /// A subject has been synchronized
    StateUpdated {
        sn: u64,
        subject_id: String,
    },
    // Approval Received
    ApprovalReceived {
        id: String,
        subject_id: String,
        sn: u64,
    },
    /// Approval Obsoleted because gov version changed or event confirmed without us
    ObsoletedApproval {
        id: String,
        subject_id: String,
        sn: u64,
    },
    UnrecoverableError {
        error: String,
    },
}

impl From<Notification> for TapleNotification {
    fn from(value: Notification) -> Self {
        match value {
            Notification::NewEvent { sn, subject_id } => TapleNotification::NewEvent {
                sn: sn,
                subject_id: subject_id,
            },
            Notification::ApprovalReceived { id, subject_id, sn } => {
                TapleNotification::ApprovalReceived {
                    id: id,
                    subject_id: subject_id,
                    sn: sn,
                }
            }
            Notification::NewSubject { subject_id } => TapleNotification::NewSubject {
                subject_id: subject_id,
            },
            Notification::ObsoletedApproval { id, subject_id, sn } => {
                TapleNotification::ObsoletedApproval {
                    id: id,
                    subject_id: subject_id,
                    sn: sn,
                }
            }
            Notification::StateUpdated { sn, subject_id } => TapleNotification::StateUpdated {
                sn: sn,
                subject_id: subject_id,
            },
            Notification::UnrecoverableError { error } => {
                TapleNotification::UnrecoverableError { error }
            }
        }
    }
}
