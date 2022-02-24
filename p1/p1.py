import sys

class Skill:
    def __repr__(self):
        return str(self)

    def __str__(self):
        return f"{self.__class__.__name__}({self.__dict__})"

    def __init__(self, name, level):
        self.name = name
        self.level = level

    def fullfills(self, other_skill, has_mentor=False):
        if not (self.name == other_skill.name):
            return False
        if has_mentor:
            return (self.level + 1) >= other_skill.level 
        else:
            return self.level >= other_skill.level

class Contributor:
    def __repr__(self):
        return str(self)

    def __str__(self):
        return f"{self.__class__.__name__}({self.__dict__})"

    def __init__(self, name, skills):
        self.name = name
        self.skills = skills
        self.projects = []
        self.busy = False
        self.finish_day = 0

class Role:
    def __repr__(self):
        return str(self)

    def __str__(self):
        return f"{self.__class__.__name__}({self.__dict__})"

    def __init__(self, required_skill, mentor):
        self.required_skill = required_skill
        self.mentor = mentor

class Project:
    def __repr__(self):
        return str(self)

    def __str__(self):
        return f"{self.__class__.__name__}({self.__dict__})"

    def __init__(self, name, duration_days, score, best_before_day, roles):
        self.name = name
        self.duration_days = duration_days
        self.score = score
        self.best_before_day = best_before_day
        self.roles = roles
        self.finished = False

    def can_take_basic(self, devs, day):
        out_devs = []
        role_len = len(self.roles)
        for p_skill in self.roles:
            role_assigned = False
            for dev in devs:
                if dev.busy:
                    print("{} is busy!".format(dev.name))
                    continue
                for d_skill in dev.skills:
                    if d_skill.fullfills(p_skill):
                        out_devs.append(dev)
                        dev.busy = True
                        dev.finish_day = day + self.duration_days
                        role_assigned = True
                        break
                if role_assigned:
                    break

        if len(out_devs) == role_len:
            return out_devs
        else:
            return set()

 
class ProjectAssignment:
    def __repr__(self):
        return str(self)

    def __str__(self):
        return f"{self.__class__.__name__}({self.__dict__})"

    def __init__(self, name, contributors):
        self.name = name
        self.contributors = contributors
    
    def print(self):
        print(self.name)
        print(" ".join([contributor.name for contributor in self.contributors]))

        


if __name__ == "__main__":
    counts = input().split(" ")
    cont_count = int(counts[0])
    proj_count = int(counts[1])
    contributors = []
    projects = []
    for _ in range(0, cont_count):
        name, skill_count = tuple(input().split(" "))
        skills = []
        for _ in range(0, int(skill_count)):
            skill_name, level = tuple(input().split(" "))
            skill = Skill(skill_name, int(level))
            skills.append(skill)
        contributor = Contributor(name, skills)
        contributors.append(contributor)
    
    for _ in range(0, proj_count):
        name, days, score, best_before, role_count = tuple(input().split(" "))
        roles = []
        for _ in range(0, int(role_count)):
            skill_name, skill = tuple(input().split(" "))
            roles.append(Skill(skill_name, int(skill)))
        projects.append(Project(name, int(days), int(score), int(best_before), roles))

    
    day = 0
    assignments = []
    free = contributors
    projects.sort(key=lambda proj: proj.score)

    def projects_available(projects, day):
        for project in projects:
            if (day + project.duration_days) <= project.best_before_day + 100:
                return True
        return False

    while projects_available(projects, day):
        taken = set()

        for project in projects:
            if project.finished == True:
                continue

            devs = project.can_take_basic(contributors, day)
            if not len(devs) == 0 and (day + project.duration_days) <= project.best_before_day:
                assignments.append(ProjectAssignment(project.name, devs))
                project.finished = True
            
        for contributor in contributors:
            if day >= contributor.finish_day:
                contributor.busy = False
        day += 1

    print(len(assignments))
    for assignment in assignments:
        assignment.print()
    