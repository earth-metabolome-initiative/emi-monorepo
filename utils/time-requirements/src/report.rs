//! Submodule defining the structs and methods for generating a report.

use std::{collections::HashMap, io::Write, path::Path};

use chrono::{NaiveDateTime, TimeDelta};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use plotters::prelude::*;
use tabled::{settings::Style, Table, Tabled};

use crate::{prelude::TimeTracker, task::CompletedTask};

/// A report for a time tracker.
pub struct Report {
    /// The time tracker to generate a report for.
    time_tracker: TimeTracker,
    /// The previous reports, if any.
    previous_reports: Vec<TimeTracker>,
}

#[derive(Tabled)]
struct TableRow<'a> {
    name: &'a str,
    time: String,
    percentage: String,
    comment: String,
}

impl Report {
    /// Creates a new report for the given time tracker.
    pub fn new(time_tracker: TimeTracker) -> Self {
        Self { time_tracker, previous_reports: Vec::new() }
    }

    /// Adds a previous report to the current report.
    pub fn add_previous_report(&mut self, report: TimeTracker) {
        self.previous_reports.push(report);

        // We ensure that the previous reports are sorted by the start time, so
        // that they are all sorted by the time they were created.
        self.previous_reports.sort_by_key(|report| report.start());
    }

    /// Adds a directory from which to load previous reports in JSON format.
    pub fn add_directory(&mut self, directory: &std::path::Path) -> std::io::Result<()> {
        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let file = std::fs::File::open(entry.path())?;
            let report: TimeTracker = serde_json::from_reader(file)?;
            self.add_previous_report(report);
        }
        Ok(())
    }

    fn title(&self, depth: usize) -> String {
        format!("{} Time Report for {}\n\n", "#".repeat(depth + 1), self.time_tracker.name())
    }

    fn description(&self) -> String {
        let total_time = self.time_tracker.total_time();

        format!(
            "The total time spent on all tasks was {}.\n",
            HumanTime::from(total_time).to_text_en(Accuracy::Rough, Tense::Present),
        )
    }

    fn slowest_task_description(&self) -> Option<String> {
        self.time_tracker.slowest_task().map(|task| {
            let total_time = self.time_tracker.total_time();
            format!(
                "The slowest task was `{}` which took {} ({:.2}% of all time).",
                task.name(),
                HumanTime::from(task.time()).to_text_en(Accuracy::Rough, Tense::Present),
                task.time().num_seconds() as f64 / total_time.num_seconds() as f64 * 100.0,
            )
        })
    }

    /// Returns, if any, the same task from the previous reports.
    fn previous_reports_task(&self, task: &CompletedTask) -> Vec<&CompletedTask> {
        self.previous_reports
            .iter()
            .filter_map(|report| report.tasks().find(|t| t.name() == task.name()))
            .collect()
    }

    /// Returns the time requirements series for the provided task.
    fn task_series(&self, task: &CompletedTask) -> Vec<(NaiveDateTime, TimeDelta)> {
        let mut series = Vec::new();
        for report in self.previous_reports.iter().chain(std::iter::once(&self.time_tracker)) {
            for t in report.tasks() {
                if t.name() == task.name() {
                    series.push((report.start(), t.time()));
                }
            }
        }
        series.push((self.time_tracker.start(), task.time()));
        series
    }

    /// Returns, if any, the same task from the most recent previous report.
    fn previous_report_task(&self, task: &CompletedTask) -> Option<&CompletedTask> {
        self.previous_reports_task(task).last().copied()
    }

    /// Returns the comment for a given task based on the previous reports.
    fn task_comment(&self, task: &CompletedTask) -> Option<String> {
        self.previous_report_task(task).map(|previous_task| {
            let previous_time = previous_task.time();
            let current_time = task.time();
            let difference = current_time - previous_time;
            let percentage =
                difference.num_seconds() as f64 / previous_time.num_seconds() as f64 * 100.0;
            if percentage.is_nan() || percentage.abs() < 1.0 {
                return "Unchanged.".to_string();
            }
            format!(
                "{:.2}% {}",
                percentage.abs(),
                if difference.num_seconds().is_negative() { "faster" } else { "slower" },
            )
        })
    }

    /// Returns a reference to the oldest report
    pub fn oldest_report(&self) -> &TimeTracker {
        self.previous_reports.first().unwrap_or(&self.time_tracker)
    }

    /// Slowest task in the report
    pub fn slowest_task(&self) -> Option<&CompletedTask> {
        self.previous_reports
            .iter()
            .chain(std::iter::once(&self.time_tracker))
            .max_by_key(|report| report.slowest_task().map(|task| task.time()).unwrap_or_default())?
            .slowest_task()
    }

    /// Returns the total number of reports
    pub fn total_reports(&self) -> usize {
        self.previous_reports.len() + 1
    }

    /// Writes out a plot of the trends for all tasks in the report and saves it
    /// to the given file.
    pub fn plot(&self, path: &Path) -> std::io::Result<()> {
        let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                0.0..self.total_reports() as f64,
                0.0..self
                    .slowest_task()
                    .map(|task| task.time().num_seconds() as f64)
                    .unwrap_or(1.0),
            )
            .unwrap();

        chart.configure_mesh().x_labels(5).y_labels(10).draw().unwrap();

        let styles: HashMap<&CompletedTask, ShapeStyle> = self
            .time_tracker
            .tasks()
            .enumerate()
            .map(|(i, task)| (task, Palette99::pick(i).filled()))
            .collect();

        for task in self.time_tracker.tasks() {
            let series = self.task_series(task);
            chart
                .draw_series(LineSeries::new(
                    series.iter().enumerate().map(|(x, (_, y))| {
                        let y = y.num_seconds() as f64;
                        (x as f64, y)
                    }),
                    styles.get(task).unwrap().clone(),
                ))
                .unwrap();
        }

        Ok(())
    }

    /// Returns an iterator over the sub-reports.
    fn sub_reports(&self) -> impl Iterator<Item = Report> + '_ {
        self.time_tracker.sub_trackers().into_iter().cloned().map(|time_tracker| {
            Self {
                previous_reports: self
                    .previous_reports
                    .iter()
                    .filter_map(|report| report.sub_tracker_by_name(time_tracker.name()).cloned())
                    .collect(),
                time_tracker,
            }
        })
    }

    /// Returns the text of the report.
    fn text(&self, depth: usize) -> String {
        let total_time = self.time_tracker.total_time().num_seconds() as f64;
        let rows = self.time_tracker.tasks().map(|task| {
            TableRow {
                name: task.name(),
                time: HumanTime::from(task.time()).to_text_en(Accuracy::Rough, Tense::Present),
                percentage: format!(
                    "{:.2}%",
                    task.time().num_seconds() as f64 / total_time * 100.0
                ),
                comment: self.task_comment(task).unwrap_or_default(),
            }
        });
        let mut table = Table::new(rows);
        table.with(Style::markdown());

        let mut report = String::new();

        report.push_str(&self.title(depth));
        report.push_str(&self.description());

        if let Some(description) = self.slowest_task_description() {
            report.push_str(&description);
        }

        report.push_str("\n\n");
        report.push_str(&table.to_string());

        for sub_report in self.sub_reports() {
            report.push_str("\n\n");
            report.push_str(&sub_report.text((depth + 1).min(6)));
        }

        report
    }

    /// Writes out the markdown report to a given file.
    pub fn write(&self, report_path: &Path, plot_path: &Path) -> std::io::Result<()> {
        let mut file = std::fs::File::create(report_path)?;
        self.plot(plot_path)?;

        writeln!(file, "{}", self.text(0))?;
        writeln!(file, "\n![Plot]({})", plot_path.to_string_lossy())?;

        Ok(())
    }
}
