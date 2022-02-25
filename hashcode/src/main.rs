use std::{io::BufRead, str::Split};

use contributors::Contributor;
use skill::Skill;

use crate::contributors::Contributors;

mod contributors;
mod skill;

#[derive(Debug, Clone)]
struct Project {
    name: String,
    duration_days: isize,
    score: isize,
    best_before_day: isize,
    roles: Vec<Skill>,
    finished: bool,
}

impl Project {
    fn can_take_basic<'a>(
        &self,
        contributors: &'a mut Vec<Contributor>,
        current_day: isize,
    ) -> Option<Vec<Contributor>> {
        let mut devs = Vec::new();

        let mut contributor_map: Vec<(&Contributor, bool)> = contributors
            .iter()
            .map(|contributor| (contributor, false))
            .collect();

        for skill in &self.roles {
            'this_skill: for (contrib, used) in contributor_map.iter_mut() {
                if *used || !contrib.done(current_day) {
                    continue;
                }

                for contrib_skill in contrib.skills() {
                    if contrib_skill.fullfills(skill) {
                        let skill_to_upgrade = if contrib_skill.level() == skill.level() {
                            Some(contrib_skill.name().clone())
                        } else {
                            None
                        };

                        devs.push((contrib.clone(), skill_to_upgrade));
                        *used = true;
                        break 'this_skill;
                    }
                }
            }
        }

        if devs.len() == self.roles.len() {
            for contributor in &devs {
                let found_contributor = contributors
                    .iter_mut()
                    .find(|dev| dev.name() == contributor.0.name())
                    .unwrap();
                found_contributor.set_finish_day(Some(current_day + self.duration_days));
                if let Some(skill_upgr) = &contributor.1 {
                    found_contributor.upgrade_skill(skill_upgr);
                }
            }

            Some(devs.iter().map(|(cont, _)| cont.clone()).collect())
        } else {
            None
        }
    }

    fn find_devs_for(
        &self,
        contributors: &mut Contributors,
        today: isize,
    ) -> Option<Vec<Contributor>> {
        let mut devs = Vec::with_capacity(self.roles.len());
        let mut assignments = Vec::with_capacity(self.roles.len());
        for role in &self.roles {
            if let Some((contributor, mentored)) =
                contributors.find_contributor_for_skill(&devs, &role)
            {
                devs.push(contributor);
                assignments.push((role, mentored));
            }
        }
        if devs.len() == self.roles.len() {
            for (dev, (role, _mentored)) in devs.iter_mut().zip(assignments.iter()) {
                dev.set_finish_day(Some(today + self.duration_days));
                if role.level()
                    >= dev
                        .find_skill(role.name())
                        .expect("Didn't find skill")
                        .level()
                {
                    dev.upgrade_skill(role.name());
                }
            }

            Some(devs)
        } else {
            devs.into_iter()
                .for_each(|dev| contributors.put_contributor(dev));

            None
        }
    }
}

#[derive(Debug, Clone)]
struct ProjectAssignment {
    project: Project,
    contributors: Vec<String>,
}

impl ProjectAssignment {
    fn print(&self) {
        println!("{}", self.project.name);

        let mut total = 0;
        for contributor in &self.contributors {
            print!("{}", contributor);
            total += 1;
            if total != self.contributors.len() {
                print!(" ");
            }
        }
        println!();
    }
}

fn projects_available(projects: &Vec<Project>, current_day: isize) -> bool {
    true
}

#[allow(unused)]
fn traditional(
    mut projects: Vec<Project>,
    mut contributors: Vec<Contributor>,
) -> Vec<ProjectAssignment> {
    let mut assignments = Vec::new();
    let mut day = 0;
    while projects_available(&projects, day) {
        let mut dt = None;
        for project in &mut projects {
            if project.finished {
                continue;
            }

            if (day + project.duration_days) <= project.best_before_day {
                if let Some(devs) = project.can_take_basic(&mut contributors, day) {
                    project.finished = true;
                    dt = Some(if let Some(dt) = dt {
                        if project.duration_days < dt {
                            project.duration_days
                        } else {
                            dt
                        }
                    } else {
                        project.duration_days
                    });
                    assignments.push(ProjectAssignment {
                        project: project.clone(),
                        contributors: devs
                            .into_iter()
                            .map(|contrib| contrib.name().clone())
                            .collect(),
                    });
                }
            }
        }

        for contributor in contributors.iter_mut() {
            if let Some(finish_day) = contributor.finish_day() {
                if &day >= finish_day {
                    contributor.set_finish_day(None);
                }
            }
        }
        eprintln!(
            "{}, {}/{}",
            day,
            projects.iter().filter(|proj| proj.finished).count(),
            projects.len(),
        );
        if let Some(dt) = dt {
            day += dt;
        } else {
            day += 1;
        }
    }

    assignments
}

fn new(mut projects: Vec<Project>, mut contributors: Contributors) -> Vec<ProjectAssignment> {
    let mut assignments = Vec::new();
    let mut currently_assigned = Vec::new();

    let projects_total = projects.len();
    let mut ongoing_projects = Vec::new();

    let mut today = 0;
    while projects_available(&projects, today) {
        eprintln!(
            "{}, projects done: {}/{}. Ongoing: {}",
            today,
            assignments.len(),
            projects_total,
            ongoing_projects.len(),
        );

        let mut dt = None;

        projects = projects
            .into_iter()
            .filter_map(|project| {
                if today + project.duration_days < project.best_before_day + project.score {
                    if let Some(mut devs) = project.find_devs_for(&mut contributors, today) {
                        assignments.push(ProjectAssignment {
                            project: project.clone(),
                            contributors: devs
                                .iter()
                                .map(|contrib| contrib.name().clone())
                                .collect(),
                        });
                        currently_assigned.append(&mut devs);
                        ongoing_projects.push((project.clone(), today));

                        if let Some(duration) = dt {
                            if duration > project.duration_days {
                                dt = Some(project.duration_days);
                            }
                        } else {
                            dt = Some(project.duration_days);
                        }
                        None
                    } else {
                        Some(project)
                    }
                } else {
                    None
                }
            })
            .collect();

        ongoing_projects.retain(|(project, start_day)| today - start_day <= project.duration_days);

        if dt.is_none() {
            ongoing_projects.iter().for_each(|(proj, start_date)| {
                let duration = proj.duration_days - (today - start_date);
                if duration > 0 {
                    if let Some(dur) = dt {
                        if dur > duration {
                            dt = Some(duration);
                        }
                    } else {
                        dt = Some(duration);
                    }
                }
            })
        }

        currently_assigned = currently_assigned
            .into_iter()
            .filter_map(|dev| {
                if dev.done(today) {
                    contributors.put_contributor(dev);
                    None
                } else {
                    Some(dev)
                }
            })
            .collect();

        today += if let Some(dt) = dt {
            dt
        } else if ongoing_projects.len() == 0 {
            break;
        } else {
            1
        };
    }

    assignments
}

fn main() {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();

    let mut take_line = || lines.next().unwrap().unwrap();

    fn to_isize(input: &mut Split<&str>) -> isize {
        isize::from_str_radix(input.next().unwrap(), 10).unwrap()
    }

    let counts_line = take_line();
    let counts = &mut counts_line.split(" ");
    let cont_count = to_isize(counts);
    let proj_count = to_isize(counts);

    let mut contributors = Vec::new();
    let mut projects = Vec::new();

    for _ in 0..cont_count {
        let name_skill_count = take_line();
        let name_skill_count = &mut name_skill_count.split(" ");
        let name = name_skill_count.next().unwrap().to_string();
        let skill_count = to_isize(name_skill_count);

        let mut skills = Vec::new();
        for _ in 0..skill_count {
            let skill_level = take_line();
            let mut skill_level = skill_level.split(" ");
            skills.push(Skill::new(
                skill_level.next().unwrap().to_string(),
                to_isize(&mut skill_level),
            ));
        }

        contributors.push(Contributor::new(name, skills));
    }

    for _ in 0..proj_count {
        let data = take_line();
        let data = &mut data.split(" ");

        let name = data.next().unwrap();

        let duration_days = to_isize(data);
        let score = to_isize(data);
        let best_before_day = to_isize(data);
        let role_count = to_isize(data);
        let mut roles = Vec::new();

        for _ in 0..role_count {
            let skill_name_level = take_line();
            let mut skill_name_level = skill_name_level.split(" ");
            let name = skill_name_level.next().unwrap().to_string();
            let level = to_isize(&mut skill_name_level);

            roles.push(Skill::new(name, level))
        }

        projects.push(Project {
            name: name.to_string(),
            duration_days,
            score,
            best_before_day,
            roles,
            finished: false,
        })
    }

    projects.sort_by(|a, b| a.best_before_day.cmp(&b.best_before_day));

    let contributors: Contributors = contributors.into();

    let assignments = new(projects, contributors);

    println!("{}", assignments.len());
    for assignment in &assignments {
        assignment.print();
    }
}
