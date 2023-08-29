/*
----> ЗАДАНИЕ 2 "Площадь квадрата"

Создать структуру Rect (квадрат), которая задается координатами левого верхнего угла и длиной стороны.
Добавить для этой структуры методы new(top_left: (f32, f32), width: f32) -> Rect
                                   bottom_right -> (f32, f32), // Выводит координаты правого нижнего угла
                                   area -> f32 // Вычисляет площадь квадрата
                                   perimeter -> f32 // Вычисляет периметр квадрата

 */

 fn main() {
    // Создание квадрата
    let top_left = (0.0, 0.0);
    let width = 5.0;
    let square = Rect::new(top_left, width);

    // Получение координат правого нижнего угла
    let bottom_right = square.bottom_right();
    println!("Bottom right: {:?}", bottom_right);

    // Вычисление площади
    let area = square.area();
    println!("Area: {}", area);

    // Вычисление периметра
    let perimeter = square.perimeter();
    println!("Perimeter: {}", perimeter);
}

struct Rect {
    top_left: (f32, f32),
    width: f32,
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        Rect { top_left, width }
    }

    fn bottom_right(&self) -> (f32, f32) {
        let bottom_right_x = self.top_left.0 + self.width;
        let bottom_right_y = self.top_left.1 - self.width;
        (bottom_right_x, bottom_right_y)
    }

    fn area(&self) -> f32 {
        self.width * self.width
    }

    fn perimeter(&self) -> f32 {
        4.0 * self.width
    }
}
 
 
 // ----> TESTS
 #[cfg(test)]
 mod tests {
     use crate::Rect;
 
     #[test]
     fn bottom_right() {
         let rect = Rect::new((1., 2.), 5.);
 
         assert_eq!((6., -3.), rect.bottom_right())
     }
 
     #[test]
     fn area() {
         let rect = Rect::new((1., 2.), 5.);
 
         assert_eq!(25., rect.area())
     }
 
     #[test]
     fn perimeter() {
         let rect = Rect::new((1., 2.), 5.);
 
         assert_eq!(20., rect.perimeter())
     }
 }
 