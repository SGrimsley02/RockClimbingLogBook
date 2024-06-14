use futures::executor::block_on;
use sea_orm::*;
pub mod entities;
use entities::{prelude::*, *};

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
const DB_NAME: &str = "routes_db";

#[derive(Clone)]
pub struct RoutesDb {
    db: DatabaseConnection,
    db_name: String,
}



impl RoutesDb {
    pub async fn new() -> Result<RoutesDb, DbErr> {
        let db = Database::connect(DATABASE_URL).await?;
        let db = &match db.get_database_backend() {
            DbBackend::MySql => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
                ))
                .await?;
            
                let url = format!("{}/{}", DATABASE_URL, DB_NAME);
                Database::connect(&url).await?
            }
            DbBackend::Postgres => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
                ))
                .await?;
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", DB_NAME),
                ))
                .await?;
            
                let url = format!("{}/{}", DATABASE_URL, DB_NAME);
                Database::connect(&url).await?
            }
            DbBackend::Sqlite => db,
        };
        Ok(
        RoutesDb {
            db: db.clone(),
            db_name: DB_NAME.to_string(),
        }
        )
    }

    pub async fn add_grade(self, yosemite: &str, font: &str, french: &str, hueco: &str, uiaa: &str) -> Result<(), DbErr> {
        let new_grade = grades::ActiveModel {
            yosemite: ActiveValue::Set(yosemite.to_owned()),
            font: ActiveValue::Set(font.to_owned()),
            french: ActiveValue::Set(french.to_owned()),
            hueco: ActiveValue::Set(hueco.to_owned()),
            uiaa: ActiveValue::Set(uiaa.to_owned()),
            ..Default::default()
        };
        Grades::insert(new_grade).exec(&self.db).await?;
        Ok(())
    }

    pub async fn remove_grade(self, id: i32) -> Result<(), DbErr> {
        let delete_grade = grades::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        };
        delete_grade.delete(&self.db).await?;
        Ok(())
    }

    pub async fn get_grade_id(self, grd: &str) -> Result<i32, DbErr> {
        let grade = Grades::find().filter(grades::Column::Yosemite.eq(grd)).one(&self.db).await?;
        // Return the id of the grade
        Ok(grade.unwrap().id)
    }

    pub async fn get_grade(self, id: u16) -> Result<grades::Model, DbErr> {
        let grade = Grades::find_by_id(id as i32).one(&self.db).await?;
        Ok(grade.unwrap())
    }

    pub async fn add_route(self, name: String, length: i32, pitches: i32, style: String, grade_id: i32) -> Result<(), DbErr> {
        let new_route = routes::ActiveModel {
            name: ActiveValue::Set(name.to_owned()),
            length: ActiveValue::Set(length),
            pitches: ActiveValue::Set(pitches),
            style: ActiveValue::Set(style.to_owned()),
            grade_id: ActiveValue::Set(grade_id),
            ..Default::default()
        };
        Routes::insert(new_route).exec(&self.db).await?;
        Ok(())
    }

    pub async fn remove_route(self, id: i32) -> Result<(), DbErr> {
        let delete_route = routes::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        };
        delete_route.delete(&self.db).await?;
        Ok(())
    }

    pub async fn find_route_name(self, name: &str) -> Result<Option<routes::Model>, DbErr> {
        let route = Routes::find().filter(routes::Column::Name.eq(name)).one(&self.db).await?;
        Ok(route)
    }

    pub async fn get_route_id(self, name: &str) -> Result<i32, DbErr> {
        let route = Routes::find().filter(routes::Column::Name.eq(name)).one(&self.db).await?;
        Ok(route.unwrap().id)
    }

    pub async fn find_routes_by_grade(self, grade: i32) -> Result<Vec<String>, DbErr> {
        let grades: Vec<grades::Model> = Grades::find()
            .filter(grades::Column::Id.eq(grade))
            .all(&self.db)
            .await?;

        let find_routes: Vec<Vec<routes::Model>> = grades.load_many(Routes, &self.db).await?;
        let mut routes_at_grade: Vec<String> = find_routes[0].to_owned().into_iter().map(|route| route.name.clone()).collect();
        routes_at_grade.sort_unstable();
        Ok(routes_at_grade)
    }

    pub async fn find_all_routes(self) -> Result<Vec<routes::Model>, DbErr> {
        let all_routes: Vec<routes::Model> = Routes::find().all(&self.db).await?;
        //let mut all_route_names: Vec<String> = all_routes.into_iter().map(|route| route.name.clone()).collect();
        //all_route_names.sort_unstable();
        Ok(all_routes)
    }

    pub async fn run_db(self) -> Result<(), DbErr> { //Currently using this mostly just to test some features
        // Connect to the database

        /*
        // Add a grade
        add_grade("5.1", "1", "2", "VB", "II").await?;

        // Delete a grade
        remove_grade(7).await?;
        
        // Add a route
        add_route("New Test Route 2", 10, 1, "Trad", 5).await?;

        // Delete a route
        remove_route(6).await?;

        // Find all grades
        let all_grades: Vec<grades::Model> = Grades::find().all(&self.db).await?;
        println!("{:?}", all_grades.len());

        // Find grade by id
        let some_grade: Option<grades::Model> = Grades::find_by_id(1).one(&self.db).await?;
        println!("YDS Grade: {}", some_grade.unwrap().yosemite);

        // Find grade by non-id column
        let some_other_grade: Option<grades::Model> = Grades::find().filter(grades::Column::Yosemite.eq("5.0")).one(&self.db).await?;
        println!("Yosemite 5.0 = French {}", some_other_grade.unwrap().french);

        // Find route name
        let some_route: Option<routes::Model> = find_route_name("New Test Route 2").await?;
        println!("Route: {:?}", some_route.unwrap());

        // Find routes by grade
        let routes_at_grade: Vec<String> = find_routes_by_grade(5).await?;
        println!("Routes at grade 5: {:?}", routes_at_grade);
        */

        self.clone().add_grade("5.0", "(0)", "0", "(VB)", "I").await?;
        self.clone().add_grade("5.1", "(0)", "1", "(VB)", "I").await?;
        self.clone().add_grade("5.2", "(0)", "2", "(VB)", "II").await?;
        let grade_id: i32 = self.clone().get_grade_id("5.1").await?;
        println!("Grade ID: {}", grade_id);


        println!("Successful refactor!");

        Ok(())
    }

    async fn connect() -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect(DATABASE_URL).await?;
        Ok(db.into())
    }
}