use crate::vector::Vector;

use super::{scene_objects::Color, utils::sort_vectors_by_z::sort_enumerated_vectors_by_z, Scene};

/// Result render image's shape
pub struct ImageShape {
    _aspect_ratio: f64,
    width: u16,
    height: u16,
}

impl ImageShape {
    fn new(width: u16, aspect_ratio: f64) -> Self {
        let height = u16::try_from(((width as f64) / aspect_ratio).round() as i64)
            .expect("Image's height should be not greater than 65,536 px");

        Self {
            _aspect_ratio: aspect_ratio,
            width,
            height,
        }
    }
}

/// Render params that are calculated based on scene's camera and result image properties
pub struct RenderParams {
    /// Horizontal delta vector from pixel to pixel
    pub pixel_delta_u: Vector,
    /// Vertical delta vector from pixel to pixel
    pub pixel_delta_v: Vector,
    /// Vieport's upper left corner location
    pub viewport_upper_left: Vector,
    /// Upper left pixel's location
    pub pixel00_loc: Vector,
}

impl RenderParams {
    pub fn new(
        pixel_delta_u: Vector,
        pixel_delta_v: Vector,
        viewport_upper_left: Vector,
        pixel00_loc: Vector,
    ) -> Self {
        Self {
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel00_loc,
        }
    }
}

pub struct Renderer {
    img_shape: Option<ImageShape>,
    params: Option<RenderParams>,
}

type RenderResult = Vec<Vec<[u8; 3]>>;

impl Renderer {
    pub fn new(img_shape: Option<ImageShape>, params: Option<RenderParams>) -> Self {
        Self { img_shape, params }
    }

    /// Creates an empty renderer
    pub fn empty() -> Self {
        Self {
            img_shape: None,
            params: None,
        }
    }
}

impl Scene {
    /// Populates render params if they are not defined or reconstructs them if image width has changed
    fn prepare_render(&mut self, img_width: u16) {
        if self.renderer.params.is_none()
            || self.renderer.img_shape.take().unwrap().width != img_width
        {
            let img_shape = ImageShape::new(img_width, self.camera.viewport.aspect_ratio);

            let pixel_delta_u = self.camera.viewport.viewport_u / (img_shape.width as f64);
            let pixel_delta_v = self.camera.viewport.viewport_v / (img_shape.height as f64);

            let viewport_upper_left = self.camera.camera_center
                - Vector::new(0.0, 0.0, self.camera.focal_length)
                - self.camera.viewport.viewport_u / 2.0
                - self.camera.viewport.viewport_v / 2.0;
            let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

            let render_params = Some(RenderParams::new(
                pixel_delta_u,
                pixel_delta_v,
                viewport_upper_left,
                pixel00_loc,
            ));

            self.renderer = Renderer::new(Some(img_shape), render_params);
        }
    }

    pub fn render(&mut self, img_width: u16) -> Option<RenderResult> {
        self.prepare_render(img_width);

        if let Renderer {
            img_shape: Some(img_shape),
            params: Some(params),
        } = &self.renderer
        {
            // Returns delta vector from camera center to pixel
            let get_pixel_vector = |x: u16, y: u16| -> Vector {
                let pixel_center = params.pixel00_loc
                    + params.pixel_delta_u * (x as f64)
                    + params.pixel_delta_v * (y as f64);
                let vector = pixel_center - self.camera.camera_center;

                vector
            };

            // Returns color for the given pixel (x, y)
            let get_pixel_rgb = |x: u16, y: u16| -> Color {
                let ray = get_pixel_vector(x, y);

                let mut objects_intersection_list =
                    self.objects
                        .iter()
                        .enumerate()
                        .filter_map(|(i, object)| {
                            object.calc_ray_intersection(&ray).and_then(
                                |intersection_coords_list| Some((i, intersection_coords_list[0])),
                            )
                        })
                        .collect::<Vec<(usize, Vector)>>();

                sort_enumerated_vectors_by_z(objects_intersection_list.as_mut());

                let pixel_rgb: Color = if let Some((i, _coords)) = objects_intersection_list.first()
                {
                    self.objects[*i].get_color(&ray)
                } else {
                    [0, 0, 0]
                };

                pixel_rgb
            };

            // Creates an image representation in array (`Vec<Vec<[u8; 3]>>`)
            // with given `img_width` x `img_height` size
            // calling get_pixel_rgb to obtain each pixel data
            let img_arr = (0..img_shape.height)
                .into_iter()
                .map(|y| {
                    (0..img_shape.width)
                        .into_iter()
                        .map(|x| get_pixel_rgb(x, y))
                        .collect::<Vec<Color>>()
                })
                .collect::<Vec<Vec<Color>>>();

            Some(img_arr)
        } else {
            None
        }
    }
}
