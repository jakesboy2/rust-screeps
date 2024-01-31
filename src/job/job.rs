use log::debug;

struct Job<T> {
  jobType: T
}

impl<T> Job<T> where T: JobType {
  fn debug_log(&self) {
    debug!("Begin Job<T> Debug Log");
    self.jobType.debug_log();
    debug!("End Job<T> Debug Log");
  }
}
trait JobType {
  fn debug_log(&self);
  fn delete(&self);
}

struct RoomJob;

impl JobType for RoomJob {
  fn debug_log(&self) {
    debug!("RoomJob Debug Log");
  }

  fn delete(&self) {
    debug!("RoomJob Delete");
  }
}
struct CreepJob;

impl JobType for CreepJob {
  fn debug_log(&self) {
    debug!("CreepJob Debug Log");
  }

  fn delete(&self) {
    debug!("CreepJob Delete");
  }
}