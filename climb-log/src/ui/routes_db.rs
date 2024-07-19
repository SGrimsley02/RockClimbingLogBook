//use futures::executor::block_on;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, EntityTrait, LoaderTrait, QueryFilter, QueryOrder, Statement};
pub mod entities;
use entities::{prelude::*, grades, routes, sends};

const DATABASE_URL: &str = "sqlite:./src/routes_sql.db?mode=rwc";
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
                    format!("CREATE DATABASE IF NOT EXISTS `{DB_NAME}`;"),
                ))
                .await?;
            
                let url = format!("{DATABASE_URL}/{DB_NAME}");
                Database::connect(&url).await?
            }
            DbBackend::Postgres => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{DB_NAME}\";"),
                ))
                .await?;
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{DB_NAME}\";"),
                ))
                .await?;
            
                let url = format!("{DATABASE_URL}/{DB_NAME}");
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

    // Grade Funcs
    pub async fn add_grade(self, yosemite: Option<String>, font: Option<String>, french: Option<String>, hueco: Option<String>, uiaa: Option<String>) -> Result<(), DbErr> {
        let new_grade = grades::ActiveModel {
            yosemite: ActiveValue::Set(yosemite.clone()),
            font: ActiveValue::Set(font.clone()),
            french: ActiveValue::Set(french.clone()),
            hueco: ActiveValue::Set(hueco.clone()),
            uiaa: ActiveValue::Set(uiaa.clone()),
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
        let mut grade = Grades::find().filter(grades::Column::Yosemite.eq(grd)).one(&self.db).await?;
        if grade.is_none() {
            grade = Grades::find().filter(grades::Column::Hueco.eq(grd)).one(&self.db).await?;
        }
        // Return the id of the grade
        Ok(grade.unwrap().id)
    }

    pub async fn get_grade(self, id: i32) -> Result<grades::Model, DbErr> {
        let grade = Grades::find_by_id(id).one(&self.db).await?;
        Ok(grade.unwrap())
    }

    pub async fn get_all_grades(self) -> Result<Vec<grades::Model>, DbErr> {
        let all_grades = Grades::find().all(&self.db).await?;
        Ok(all_grades)
    }

    // Route Funcs
    #[allow(clippy::too_many_arguments)]
    pub async fn add_route(self, name: String, length: i32, pitches: i32, style: String, grade_id: i32) -> Result<(), DbErr> {
        let new_route = routes::ActiveModel {
            name: ActiveValue::Set(name.clone()),
            length: ActiveValue::Set(length),
            pitches: ActiveValue::Set(pitches),
            style: ActiveValue::Set(style.clone()),
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
        if let None = route {
            return Ok(None);
        }
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
        let mut routes_at_grade: Vec<String> = find_routes[0].iter().cloned().map(|route| route.name.clone()).collect();
        routes_at_grade.sort_unstable();
        Ok(routes_at_grade)
    }

    pub async fn find_all_routes(self) -> Result<Vec<routes::Model>, DbErr> {
        let all_routes: Vec<routes::Model> = Routes::find().all(&self.db).await?;
        //let mut all_route_names: Vec<String> = all_routes.into_iter().map(|route| route.name.clone()).collect();
        //all_route_names.sort_unstable();
        Ok(all_routes)
    }

    pub async fn find_route_by_id(self, id: i32) -> Result<routes::Model, DbErr> {
        let route = Routes::find_by_id(id).one(&self.db).await?;
        Ok(route.unwrap())
    }

    pub async fn find_route_and_grade(self, name: &str) -> Result<(routes::Model, grades::Model), DbErr> {
        let route = self.clone().find_route_name(name).await?;
        if route.is_none() {
            return Err(DbErr::RecordNotFound("Route not found".to_string()));
        }
        let route = route.unwrap();
        let grade = self.clone().get_grade(route.grade_id).await?;
        Ok((route, grade))
    }

    pub async fn find_all_routes_and_grade(self) -> Result<Vec<(routes::Model, grades::Model)>, DbErr> {
        let all_routes = self.clone().find_all_routes().await?;
        let mut all_routes_and_grades: Vec<(routes::Model, grades::Model)> = Vec::new();
        for route in all_routes {
            let grade = self.clone().get_grade(route.grade_id).await?;
            all_routes_and_grades.push((route, grade));
        }
        Ok(all_routes_and_grades)
    }

    // Send/Session Funcs
    #[allow(clippy::too_many_arguments)]
    pub async fn add_send(self, session: i32, route: entities::routes::Model, date: String, partner: Option<String>, send_type: String, attempts: i32, notes: Option<String>) -> Result<(), DbErr> {
        let new_send = sends::ActiveModel {
            session: ActiveValue::Set(session),
            date: ActiveValue::Set(date.clone()),
            partner: ActiveValue::Set(partner.clone()),
            r#type: ActiveValue::Set(send_type.clone()),
            attempts: ActiveValue::Set(attempts),
            notes: ActiveValue::Set(notes.clone()),
            route: ActiveValue::Set(self.clone().get_route_id(&route.name).await.unwrap().to_owned()),
            ..Default::default()
        };

        Sends::insert(new_send).exec(&self.db).await?;
        Ok(())
    }

    pub async fn get_session(self, id: i32) -> Result<Vec<sends::Model>, DbErr> {
        let session = Sends::find().filter(sends::Column::Session.eq(id)).all(&self.db).await?;
        Ok(session)
    }

    pub async fn get_all_sends(self) -> Result<Vec<sends::Model>, DbErr> {
        let all_sends: Vec<sends::Model> = Sends::find().all(&self.db).await?;
        Ok(all_sends)
    }

    async fn remove_send(self, id: i32, session: i32) -> Result<(), DbErr> {

        let delete_send = sends::ActiveModel {
            id: ActiveValue::Set(id),
            session: ActiveValue::Set(session),
            ..Default::default()
        };
        delete_send.delete(&self.db).await?;
        Ok(())
    }

    pub async fn remove_session(self, id: i32) -> Result<(), DbErr> {
        let session = self.clone().get_session(id).await?;
        for send in session {
            self.clone().remove_send(send.id, send.session).await?;
        }
        Ok(())
    }

    pub async fn get_next_session_id(self) -> Result<i32, DbErr> {
        // Get the highest session id
        let id = Sends::find().order_by_desc(sends::Column::Session).one(&self.db).await?;
        if id.is_none() {
            return Ok(1);
        }
        Ok(id.unwrap().session + 1)
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

        //self.clone().add_grade("5.0", "(0)", "0", "(VB)", "I").await?;
        //self.clone().add_grade("5.1", "(0)", "1", "(VB)", "I").await?;
        //self.clone().add_grade("5.2", "(0)", "2", "(VB)", "II").await?;
        //self.clone().add_grade("5.0", "(0)", "1", "(VB)", "I").await?;
        let _grade_id: i32 = self.clone().get_grade_id("5.0").await?;
        

        let _session_id: i32 = self.clone().get_next_session_id().await?;
        

        Ok(())
    }

    async fn connect() -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect(DATABASE_URL).await?;
        Ok(db)
    }
}