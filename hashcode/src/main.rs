use std::{io::BufRead, str::Split};

#[derive(Debug, Clone, PartialEq)]
struct Skill {
    name: String,
    level: usize,
}

impl Skill {
    fn fullfills(&self, other: &Skill, has_mentor: bool) -> bool {
        if self.name != other.name {
            false
        } else if has_mentor {
            self.level >= other.level.min(1) - 1
        } else {
            self.level >= other.level
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Contributor {
    name: String,
    skills: Vec<Skill>,
    finish_day: Option<usize>,
}

impl Contributor {
    fn busy(&self) -> bool {
        self.finish_day.is_some()
    }
}

#[derive(Debug, Clone)]
struct Project {
    name: String,
    duration_days: usize,
    score: usize,
    best_before_day: usize,
    roles: Vec<Skill>,
    finished: bool,
}

impl Project {
    fn can_take_basic<'a>(
        &self,
        contributors: &'a mut Vec<Contributor>,
        current_day: usize,
    ) -> Option<Vec<Contributor>> {
        let mut devs = Vec::new();

        let mut contributor_map: Vec<(&Contributor, bool)> = contributors
            .iter()
            .map(|contributor| (contributor, false))
            .collect();

        for skill in &self.roles {
            'this_skill: for (contrib, used) in contributor_map.iter_mut() {
                if *used || contrib.busy() {
                    continue;
                }

                for contrib_skill in &contrib.skills {
                    if contrib_skill.fullfills(skill, false) {
                        let skill_to_upgrade = if contrib_skill.level == skill.level {
                            Some(contrib_skill.name.clone())
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
                    .find(|dev| dev.name == contributor.0.name)
                    .unwrap();
                found_contributor.finish_day = Some(current_day + self.duration_days);
                if let Some(skill_upgr) = &contributor.1 {
                    found_contributor
                        .skills
                        .iter_mut()
                        .find(|skill| &skill.name == skill_upgr)
                        .unwrap()
                        .level += 1;
                }
            }

            Some(devs.iter().map(|(cont, _)| cont.clone()).collect())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct ProjectAssignment {
    name: String,
    contributors: Vec<Contributor>,
}

impl ProjectAssignment {
    fn print(&self) {
        println!("{}", self.name);

        let mut total = 0;
        for contributor in &self.contributors {
            print!("{}", contributor.name);
            total += 1;
            if total != self.contributors.len() {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();

    let mut take_line = || lines.next().unwrap().unwrap();

    fn to_usize(input: &mut Split<&str>) -> usize {
        usize::from_str_radix(input.next().unwrap(), 10).unwrap()
    }

    let counts_line = take_line();
    let counts = &mut counts_line.split(" ");
    let cont_count = to_usize(counts);
    let proj_count = to_usize(counts);

    let mut contributors = Vec::new();
    let mut projects = Vec::new();

    for _ in 0..cont_count {
        let name_skill_count = take_line();
        let name_skill_count = &mut name_skill_count.split(" ");
        let name = name_skill_count.next().unwrap().to_string();
        let skill_count = to_usize(name_skill_count);

        let mut skills = Vec::new();
        for _ in 0..skill_count {
            let skill_level = take_line();
            let mut skill_level = skill_level.split(" ");
            skills.push(Skill {
                name: skill_level.next().unwrap().to_string(),
                level: to_usize(&mut skill_level),
            });
        }

        contributors.push(Contributor {
            name,
            skills,
            finish_day: None,
        });
    }

    for _ in 0..proj_count {
        let data = take_line();
        let data = &mut data.split(" ");

        let name = data.next().unwrap();

        let duration_days = to_usize(data);
        let score = to_usize(data);
        let best_before_day = to_usize(data);
        let role_count = to_usize(data);
        let mut roles = Vec::new();

        for _ in 0..role_count {
            let skill_name_level = take_line();
            let mut skill_name_level = skill_name_level.split(" ");
            let name = skill_name_level.next().unwrap().to_string();
            let level = to_usize(&mut skill_name_level);

            roles.push(Skill { name, level })
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

    projects.sort_by(|a, b| a.score.cmp(&b.score));

    fn projects_available(projects: &Vec<Project>, current_day: usize) -> bool {
        if current_day > 200 {
            return false;
        }
        for project in projects {
            if (current_day + project.duration_days) <= project.best_before_day {
                eprintln!(
                    "Project {} has a before day of {} at day {}",
                    project.name, project.best_before_day, current_day
                );
                return true;
            }
        }
        false
    }

    let mut assignments = Vec::new();
    let mut day = 0;
    while projects_available(&projects, day) {
        for project in &mut projects {
            if project.finished {
                continue;
            }

            if (day + project.duration_days) <= project.best_before_day {
                if let Some(devs) = project.can_take_basic(&mut contributors, day) {
                    project.finished = true;
                    assignments.push(ProjectAssignment {
                        name: project.name.clone(),
                        contributors: devs,
                    });
                }
            }
        }

        for contributor in contributors.iter_mut() {
            if let Some(finish_day) = contributor.finish_day {
                if day >= finish_day {
                    contributor.finish_day = None;
                }
            }
        }
        eprintln!(
            "{}, {}/{}",
            day,
            projects.iter().filter(|proj| proj.finished).count(),
            projects.len(),
        );
        day += 1;
    }

    println!("{}", assignments.len());
    for assignment in assignments {
        assignment.print();
    }
}
