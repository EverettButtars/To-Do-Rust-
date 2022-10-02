
use iced::{Settings};
use iced::pure::Sandbox;
use iced::pure::widget::{Button, Text, Column, Row, Container, Checkbox, TextInput};

#[derive(Debug,Clone)]
struct Task {
    name : String,

    done : bool
}


impl Task {
    fn new(name : String) -> Self{
        Task {
            name,
            done : false
        }
    }
}

#[derive(Debug, Clone)]
enum CounterMessage {

    ClearTasks,
    AddText,
    BoxChecked(bool),
    TextInputChanged(String)

}

struct Counter {

    tasksRemaining: i32,
    list: Vec<Task>,
    textInputString: String

}


pub fn main() -> iced::Result {
    Counter::run(Settings::default())
 
 }


impl Sandbox for Counter {
    type Message = CounterMessage;

    
    fn new() -> Self {
        Counter{ tasksRemaining : 0,
                 list : Vec::new(),
                 textInputString: String::new(),
            }
    }

    fn title(&self) -> String {
        String::from("To-Do List")
    }

    fn update (&mut self, message: Self::Message) {
        match message {
            CounterMessage::AddText => {
                self.list.push(Task{name:self.textInputString.clone(), done: false}); 
                
            },
            CounterMessage::ClearTasks => {
                self.list.retain(|x| x.done == true);

            },
            CounterMessage::BoxChecked(b) => self.list.retain(|x| x.done == true),
            CounterMessage::TextInputChanged(str) => self.textInputString = str,
        }
        ;
        self.tasksRemaining = self.list.len() as i32;
    }

    fn view(&self) -> iced::pure::Element<'_, Self::Message> {
        let titleLabel = Text::new("To Do List").size(40);

        let tasksRemainLabel = Text::new(format!("Tasks Remaining: {}", self.tasksRemaining));

        let addTask = Button::new("Add Task").on_press(CounterMessage::AddText);

        let clear = Button::new("Clear finished Tasks").on_press(CounterMessage::ClearTasks);
        
        let taskInput = TextInput::new("Add Tasks Here!", &self.textInputString, CounterMessage::TextInputChanged);
        
        let textRow = Row::new().push(taskInput).push(addTask); 

        let mut col = Column::new().push(titleLabel).push(tasksRemainLabel).push(clear).push(textRow).padding(10).spacing(10);
        
        let mut ts = Column::new();

        if(!self.list.is_empty())
        {
            let mut listIter = self.list.iter();
        
            for t in listIter {

                ts = ts.push(Checkbox::new( t.done, t.name.clone(),CounterMessage::BoxChecked));
           }
        }       
        let finCol = Column::new().push(col).push(ts);
        Container::new( finCol).width(iced::Length::Fill).height(iced::Length::Fill).into()
        
    }



}



