use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

fn main() -> Result<()> {
    let width = 160;
    let height = 120;
    let mut keystate = [false; 255];

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;

    let mut canvas = Canvas::new(width, height, &window)?;

    let edges = vec![
        Point::new(0, 0),
        Point::new((width-1) as i32, 0),
        Point::new(0, (height-1) as i32),
        Point::new((width-1) as i32, (height-1) as i32),
        ];
    
    let start = Point::new(11, 7);
    let end = Point::new(40, 60);

    let a = Point::new(40, 15);
    let b = Point::new(60, 60);
    let c = Point::new(44, 44);

    let midpoint = Point::new(50, 50);
    let radius = 30;
    
    for point in edges {
        canvas.draw_pixel(point, Rgb::RED);
    }

    canvas.draw_line(start, end, Rgb::BLUE);
    canvas.draw_triangle(a, b, c, Rgb::GREEN);
    canvas.draw_circle(midpoint, radius, Rgb::RED);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                canvas.resize_surface(size.width, size.height).unwrap();
            }
            WindowEvent::CloseRequested => {
                control_flow.set_exit();
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => match state {
                ElementState::Pressed => {
                    keystate[keycode as usize] = true;
                }
                ElementState::Released => {
                    keystate[keycode as usize] = false;
                }
            },
            _ => (),
        },
        Event::MainEventsCleared => {
            if keystate[VirtualKeyCode::Escape as usize] {
                control_flow.set_exit();
            }
    
            canvas.render().unwrap();
        }
        _ => (),
    });
}

#[derive(Debug, Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    const BLACK: Rgb = Rgb::new(0, 0, 0);
    const WHITE: Rgb = Rgb::new(0xff, 0xff, 0xff);

    const RED: Rgb = Rgb::new(0xff, 0, 0);
    const ORANGE: Rgb = Rgb::new(0xff, 0x80, 0);
    const YELLOW: Rgb = Rgb::new(0xff, 0xff, 0);
    const CHARTREUSE: Rgb = Rgb::new(0x80, 0xff, 0);
    const GREEN: Rgb = Rgb::new(0, 0xff, 0);
    const SPRING_GREEN: Rgb = Rgb::new(0, 0xff, 0x80);
    const CYAN: Rgb = Rgb::new(0, 0xff, 0xff);
    const AZURE: Rgb = Rgb::new(0, 0x80, 0xff);
    const BLUE: Rgb = Rgb::new(0, 0, 0xff);
    const VIOLET: Rgb = Rgb::new(0x80, 0, 0xff);
    const MAGENTA: Rgb = Rgb::new(0xff, 0, 0xff);
    const ROSE: Rgb = Rgb::new(0xff, 0, 0x80);

    const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Debug)]
struct Canvas {
    width: u32,
    height: u32,
    pixels: Pixels,
}

impl Canvas {
    fn new(width: u32, height: u32, window: &Window) -> Result<Self> {
        let surface_texture = SurfaceTexture::new(width, height, &window);
        let pixels = Pixels::new(width, height, surface_texture)?;
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    fn draw_pixel(&mut self, point: Point, color: Rgb) {
        dbg!(point);
        let frame = self.pixels.frame_mut();
        let idx = 4 * (point.y * self.width as i32 + point.x) as usize;
        frame[idx] = color.r;
        frame[idx + 1] = color.g;
        frame[idx + 2] = color.b;
        frame[idx + 3] = 0xff;
    }

    fn draw_line(&mut self, start: Point, end: Point, color: Rgb) {
        
        let dx = (end.x - start.x).abs();
        let dy = (end.y - start.y).abs();

        let sx = { if start.x < end.x { 1 } else { -1 } };
        let sy = { if start.y < end.y { 1 } else { -1 } };

        let mut error = (if dx > dy  { dx } else { -dy }) / 2;
        
        let mut point = start.clone();

        loop {
            self.draw_pixel(point, color);
            if point.x == end.x && point.y == end.y {
                break;
            }
            let error2 = error;
            if error2 > -dx {
                error -= dy;
                point.x += sx;
            }
            if error2 < dy {
                error += dx;
                point.y += sy;
            }
        }
    }

    fn draw_circle(&mut self, midpoint: Point, radius: i32, color: Rgb) {
        let mut radius = radius;
        let mut point = Point::new(-radius, 0);
        let mut error = 2 - 2 * radius;

        loop {
            self.draw_pixel(Point::new(midpoint.x - point.x, midpoint.y + point.y), color);
            self.draw_pixel(Point::new(midpoint.x - point.y, midpoint.y - point.x), color);
            self.draw_pixel(Point::new(midpoint.x + point.x, midpoint.y - point.y), color);
            self.draw_pixel(Point::new(midpoint.x + point.y, midpoint.y + point.x), color);

            radius = error;

            if radius <= point.y {
                error += (point.y + 1) * 2;
                point.y += 1;
            }
            if radius > point.x || error > point.y {
                error += (point.x + 2) * 2;
                point.x += 1;
            }
            if point.x >= 0 {
                break;
            }
        }
    }

    fn draw_triangle(&mut self, a: Point, b: Point, c: Point, color: Rgb) {
        self.draw_line(a, b, color);
        self.draw_line(b, c, color);
        self.draw_line(c, a, color);
    }

    fn render(&self) -> Result<()> {
        Ok(self.pixels.render()?)
    }

    fn resize_surface(&mut self, width: u32, height: u32) -> Result<()> {
        Ok(self.pixels.resize_surface(width, height)?)
    }
}
