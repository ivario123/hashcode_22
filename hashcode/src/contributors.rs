use std::collections::HashMap;

use crate::skill::Skill;

#[derive(Debug, Clone, PartialEq)]
pub struct Contributors {
    map: HashMap<String, Vec<(isize, String)>>,
}

impl From<Vec<Contributor>> for Contributors {
    fn from(contributors: Vec<Contributor>) -> Self {
        let mut me = Self {
            map: HashMap::new(),
        };
        for contributor in contributors {
            me.put_contributor(contributor);
        }

        me
    }
}

impl Contributors {
    pub fn find_contributor_for_skill(
        &mut self,
        onboard_devs: &Vec<Contributor>,
        sought_skill: &Skill,
    ) -> Option<(Contributor, bool)> {
        let mut found_contributor = None;
        if let Some(contrib) = self.map.get(sought_skill.name()) {
            for (level, contributor_name) in contrib {
                if *level >= sought_skill.level() {
                    found_contributor = Some(contributor_name.clone());
                    break;
                } else if *level == sought_skill.level() - 1 {
                    if onboard_devs
                        .iter()
                        .find(|dev| {
                            dev.skills
                                .iter()
                                .find(|skill| skill.fullfills(sought_skill))
                                .is_some()
                        })
                        .is_some()
                    {
                        found_contributor = Some(contributor_name.clone());
                        break;
                    }
                }
            }
        }

        found_contributor
            .map(|name| self.take_contributor(&name).map(|dev| (dev, false)))
            .flatten()
    }

    pub fn take_contributor(&mut self, contrib_name: &String) -> Option<Contributor> {
        let mut contributor: Option<Contributor> = None;
        for (skill_name, contributors) in &mut self.map {
            if let Some((index, level, name)) = contributors
                .iter()
                .enumerate()
                .find(|(_, (_, ctr_name))| ctr_name == contrib_name)
                .map(|(index, (level, name))| (index, *level, name.clone()))
            {
                contributors.remove(index);

                let skill = Skill::new(skill_name.clone(), level);
                if let Some(contributor) = &mut contributor {
                    contributor.skills.push(skill);
                } else {
                    contributor = Some(Contributor::new(name, vec![skill]));
                }
            }
        }

        contributor
    }

    pub fn put_contributor(&mut self, contributor: Contributor) {
        for skill in contributor.skills() {
            if let Some(inserted_skill) = self.map.get_mut(skill.name()) {
                inserted_skill.push((skill.level(), contributor.name().clone()));
                inserted_skill.sort_by(|a, b| a.0.cmp(&b.0));
            } else {
                self.map.insert(
                    skill.name().clone(),
                    vec![(skill.level(), contributor.name().clone())],
                );
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contributor {
    name: String,
    skills: Vec<Skill>,
    finish_day: Option<isize>,
}

impl Contributor {
    pub fn new(name: String, skills: Vec<Skill>) -> Self {
        Self {
            name,
            skills,
            finish_day: None,
        }
    }

    pub fn done(&self, today: isize) -> bool {
        if let Some(finish_day) = self.finish_day {
            today >= finish_day
        } else {
            true
        }
    }

    pub fn skills(&self) -> &Vec<Skill> {
        &self.skills
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn upgrade_skill(&mut self, name: &String) {
        if let Some(skill) = self.skills.iter_mut().find(|skill| skill.name() == name) {
            skill.inc();
        }
    }

    pub fn finish_day(&self) -> &Option<isize> {
        &self.finish_day
    }

    pub fn set_finish_day(&mut self, finish_day: Option<isize>) {
        self.finish_day = finish_day;
    }

    pub fn find_skill(&self, name: &String) -> Option<&Skill> {
        self.skills.iter().find(|skill| skill.name() == name)
    }
}
