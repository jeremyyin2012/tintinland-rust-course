-- 班级
CREATE TABLE "public"."class"
(
    "id"        uuid      NOT NULL,
    "code"      text      NOT NULL,
    "name"      text      NOT NULL,
    "kind"      numeric   NOT NULL,
    "create_dt" timestamp NOT NULL,
    "create_by" uuid      NOT NULL,
    "update_dt" timestamp NOT NULL,
    "update_by" uuid      NOT NULL,
    PRIMARY KEY ("id")
);

-- 班级-学员
CREATE TABLE "public"."class_student"
(
    "class_id"   uuid      NOT NULL,
    "student_id" uuid      NOT NULL,
    "create_dt"  timestamp NOT NULL,
    "create_by"  uuid      NOT NULL,
    "update_dt"  timestamp NOT NULL,
    "update_by"  uuid      NOT NULL,
    PRIMARY KEY ("class_id", "student_id")
);


-- 学员
CREATE TABLE "public"."student"
(
    "id"        uuid      NOT NULL,
    "code"      text      NOT NULL,
    "name"      text      NOT NULL,
    "kind"      numeric   NOT NULL,
    "create_dt" timestamp NOT NULL,
    "create_by" uuid      NOT NULL,
    "update_dt" timestamp NOT NULL,
    "update_by" uuid      NOT NULL,
    PRIMARY KEY ("id")
);


-- 成绩
CREATE TABLE "public"."grades"
(
    "id"         uuid      NOT NULL,
    "student_id" uuid      NOT NULL,
    "course_id"  uuid      NOT NULL,
    "score"      numeric   NOT NULL,
    "create_dt"  timestamp NOT NULL,
    "create_by"  uuid      NOT NULL,
    "update_dt"  timestamp NOT NULL,
    "update_by"  uuid      NOT NULL,
    PRIMARY KEY ("id")
);


-- 教师
CREATE TABLE "public"."teacher"
(
    "id"        uuid      NOT NULL,
    "code"      text      NOT NULL,
    "name"      text      NOT NULL,
    "kind"      numeric   NOT NULL,
    "create_dt" timestamp NOT NULL,
    "create_by" uuid      NOT NULL,
    "update_dt" timestamp NOT NULL,
    "update_by" uuid      NOT NULL,
    PRIMARY KEY ("id")
);


-- 课程
CREATE TABLE "public"."course"
(
    "id"        uuid      NOT NULL,
    "code"      text      NOT NULL,
    "name"      text      NOT NULL,
    "kind"      numeric   NOT NULL,
    "create_dt" timestamp NOT NULL,
    "create_by" uuid      NOT NULL,
    "update_dt" timestamp NOT NULL,
    "update_by" uuid      NOT NULL,
    PRIMARY KEY ("id")
);


-- 任课
CREATE TABLE "public"."teach"
(
    "id"         uuid      NOT NULL,
    "kind"       numeric   NOT NULL,
    "year"       numeric   NOT NULL,
    "semester"   numeric   NOT NULL,
    "course_id"  uuid      NOT NULL,
    "teacher_id" uuid      NOT NULL,
    "create_dt"  timestamp NOT NULL,
    "create_by"  uuid      NOT NULL,
    "update_dt"  timestamp NOT NULL,
    "update_by"  uuid      NOT NULL,
    PRIMARY KEY ("id")
);


-- 配课
CREATE TABLE "public"."selection"
(
    "id"        uuid      NOT NULL,
    "class_id"  uuid      NOT NULL,
    "teach_id"  uuid      NOT NULL,
    "create_dt" timestamp NOT NULL,
    "create_by" uuid      NOT NULL,
    "update_dt" timestamp NOT NULL,
    "update_by" uuid      NOT NULL,
    PRIMARY KEY ("id")
);


-- 上课
CREATE TABLE "public"."schedule"
(
    "id"           uuid      NOT NULL,
    "selection_id" uuid      NOT NULL,
    "start_dt"     timestamp NOT NULL,
    "end_dt"       timestamp NOT NULL,
    "classroom_id" uuid      NOT NULL,
    "create_dt"    timestamp NOT NULL,
    "create_by"    uuid      NOT NULL,
    "update_dt"    timestamp NOT NULL,
    "update_by"    uuid      NOT NULL,
    PRIMARY KEY ("id")
);


-- 考试
CREATE TABLE "public"."exam"
(
    "id"           uuid      NOT NULL,
    "code"         text      NOT NULL,
    "name"         text      NOT NULL,
    "kind"         numeric   NOT NULL,
    "selection_id" uuid      NOT NULL,
    "score"        numeric   NOT NULL,
    "create_dt"    timestamp NOT NULL,
    "create_by"    uuid      NOT NULL,
    "update_dt"    timestamp NOT NULL,
    "update_by"    uuid      NOT NULL,
    PRIMARY KEY ("id")
);
