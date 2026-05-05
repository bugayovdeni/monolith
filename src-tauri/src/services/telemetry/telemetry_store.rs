use crate::domain::telemetry::models::{ChartPoint, MeasurementSnapshot};
use std::collections::VecDeque;
use std::sync::RwLock;

pub struct TelemetryStore {
    latest: RwLock<Option<MeasurementSnapshot>>,
    history: RwLock<VecDeque<ChartPoint>>,
    max_points: usize,
}

impl TelemetryStore {
    pub fn new(max_points: usize) -> Self {
        Self {
            latest: RwLock::new(None),
            history: RwLock::new(VecDeque::new()),
            max_points,
        }
    }

    pub fn push(&self, snapshot: MeasurementSnapshot) {
        // обновляем latest
        {
            let mut latest = self.latest.write().unwrap();
            *latest = Some(snapshot.clone());
        }

        // добавляем в историю
        {
            let mut history = self.history.write().unwrap();

            history.push_back(ChartPoint {
                x: snapshot.timestamp_ms,
                y: snapshot.value,
            });

            // ограничение размера
            while history.len() > self.max_points {
                history.pop_front();
            }
        }
    }

    pub fn latest(&self) -> Option<MeasurementSnapshot> {
        self.latest.read().unwrap().clone()
    }

    pub fn history(&self) -> Vec<ChartPoint> {
        self.history.read().unwrap().iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::telemetry::models::MeasurementSnapshot;

    #[test]
    fn push_updates_latest() {
        let store = TelemetryStore::new(10);

        store.push(MeasurementSnapshot {
            timestamp_ms: 1,
            value: 10.0,
        });

        let latest = store.latest().unwrap();
        assert_eq!(latest.value, 10.0);
    }

    #[test]
    fn push_adds_history() {
        let store = TelemetryStore::new(10);

        store.push(MeasurementSnapshot {
            timestamp_ms: 1,
            value: 10.0,
        });

        store.push(MeasurementSnapshot {
            timestamp_ms: 2,
            value: 20.0,
        });

        let history = store.history();

        assert_eq!(history.len(), 2);
        assert_eq!(history[0].y, 10.0);
        assert_eq!(history[1].y, 20.0);
    }

    #[test]
    fn respects_max_points() {
        let store = TelemetryStore::new(2);

        store.push(MeasurementSnapshot {
            timestamp_ms: 1,
            value: 10.0,
        });

        store.push(MeasurementSnapshot {
            timestamp_ms: 2,
            value: 20.0,
        });

        store.push(MeasurementSnapshot {
            timestamp_ms: 3,
            value: 30.0,
        });

        let history = store.history();

        assert_eq!(history.len(), 2);
        assert_eq!(history[0].x, 2);
        assert_eq!(history[1].x, 3);
    }
}
