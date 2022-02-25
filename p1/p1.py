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
        self.project = ""


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
        for p_skill in self.roles:
            for dev in devs:
                if dev.busy:
                    continue
                for d_skill in dev.skills:
                    if d_skill.fullfills(p_skill):
                        out_devs.append(dev)
                        dev.busy = True
                        dev.finish_day = day + self.duration_days
                        dev.project = self.name
                        break
                if dev.busy:
                    break

        if len(out_devs) == len(self.roles):
            return out_devs
        else:
            for dev in out_devs:
                dev.busy = False
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
        print(
            " ".join([contributor.name for contributor in self.contributors]))


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
        projects.append(Project(name, int(days), int(
            score), int(best_before), roles))

    day = 0
    assignments = []
    projects.sort(key=lambda proj: proj.score)

    print(contributors)
    print(projects)
    exit()

    def projects_available(projects, day):
        for project in projects:
            if (day + project.duration_days) <= project.best_before_day:
                return True
        return False

    while projects_available(projects, day):
        for project in projects:
            if project.finished == True:
                continue

            devs = project.can_take_basic(contributors, day)
            if not len(devs) == 0 and (day + project.duration_days) <= project.best_before_day:
                assignments.append(ProjectAssignment(project.name, devs))
                project.finished = True

        for contributor in contributors:
            if day >= contributor.finish_day and contributor.busy:
                contributor.busy = False
        print(day, file=sys.stderr)

        finished = len([project for project in projects if project.finished == True])
        print("Finished", finished, file=sys.stderr)
        day += 1

    print(len(assignments))
    for assignment in assignments:
        assignment.print()
