use chrono::{DateTime, Datelike, Local, Months, NaiveDate, Utc};
use nanoid::nanoid;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::Display,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryManager {
    pub entries: Vec<Entry>,
    pub filter: Filter,
}
impl EntryManager {
    pub fn add_entry(&mut self, entry: Entry) -> bool {
        self.entries.push(entry);
        self.sort();
        return true;
    }
    pub fn remove_entry_by_id(&mut self, id: String) -> bool {
        let idx = self.entries.par_iter().position_any(|e| e.id == id);
        if let Some(i) = idx {
            self.entries.remove(i);
            return true;
        }
        return false;
    }
    pub fn sort(&mut self) {
        self.entries.par_sort_unstable_by(|e1, e2| {
            if e1.date > e2.date {
                return Ordering::Less;
            } else if e1.created_at < e2.created_at {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        });
    }
    ///TODO add timeperiod stuff
    /// internal function to calcuate totals of amounts based on entry_types
    fn _total(&self, entry_type: EntryType) -> f64 {
        return self
            .filtered_entries()
            .par_iter()
            .filter_map(|entry| {
                if entry.entry_type == entry_type {
                    return Some(entry.amount);
                }
                return None;
            })
            .sum();
    }
    ///sum of all expenses
    pub fn total_expenses(&self) -> f64 {
        return self._total(EntryType::Expense);
    }
    ///sum of all income
    pub fn total_income(&self) -> f64 {
        return self._total(EntryType::Income);
    }
    ///all income - all expenses
    pub fn total(&self) -> f64 {
        return self.total_income() - self.total_expenses();
    }
    pub fn filtered_entries(&self) -> Vec<Entry> {
        let data = self.entries.clone();
        return data
            .into_par_iter()
            .filter(|entry| match self.filter {
                Filter::NoFilter => true,
                Filter::ThisMonth => {
                    let now = Local::now();
                    return entry.date.month() == now.month() && entry.date.year() == now.year();
                }
                Filter::ThisYear => {
                    let year = Local::now().year();
                    return entry.date.year() == year;
                }
                Filter::LastYear => {
                    let prev_year = Local::now()
                        .checked_sub_months(Months::new(12))
                        .unwrap()
                        .year();
                    return entry.date.year() == prev_year;
                }
                Filter::LastMonth => {
                    let prev_month = Local::now()
                        .checked_sub_months(Months::new(1))
                        .unwrap()
                        .month();
                    return entry.date.month() == prev_month;
                }
                Filter::Range => todo!(),
            })
            .collect();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    NoFilter,
    ThisMonth,
    ThisYear,
    LastYear,
    LastMonth,
    //TODO implement
    Range,
}
impl Default for Filter {
    fn default() -> Self {
        Self::NoFilter
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    Expense,
    Income,
}
impl Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for EntryType {
    fn default() -> Self {
        return EntryType::Expense;
    }
}

#[derive(Debug, Clone)]
pub struct NewEntry {
    pub amount: f64,
    pub entry_type: EntryType,
    pub description: String,
    pub date: NaiveDate,
}
impl NewEntry {
    pub fn reset(&mut self) {
        *self = NewEntry::default();
    }
}
impl Default for NewEntry {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            entry_type: Default::default(),
            description: Default::default(),
            date: Local::now().date_naive(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub amount: f64,
    pub entry_type: EntryType,
    pub date: NaiveDate,
    pub description: String,
}

impl From<NewEntry> for Entry {
    fn from(value: NewEntry) -> Self {
        Self {
            id: nanoid!(),
            created_at: Utc::now(),
            amount: value.amount,
            entry_type: value.entry_type,
            date: value.date,
            description: value.description,
        }
    }
}
