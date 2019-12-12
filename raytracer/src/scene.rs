/*
MIT License

Copyright (c) 2019 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::colors::Color;
use crate::lights::AnyLightObject;
use crate::primitives::{Ray, Shape};
use crate::textures::{Texture, TextureEffects};
use crate::vector::Vec3;

pub struct Scene {
    pub camera: Box<dyn RayEmitter>,
    pub lights: Vec<Box<dyn AnyLightObject>>,
    pub objects: Vec<Box<dyn AnySceneObject>>,
    pub options: SceneOptions,
}

pub struct SceneOptions {
    pub world_color: Color,
    pub ambient_light: Option<Color>,
}

impl Default for SceneOptions {
    fn default() -> Self {
        SceneOptions {
            world_color: Color::BLACK,
            ambient_light: Some(Color::new(0.2, 0.2, 0.2)),
        }
    }
}

pub trait AnySceneObject {
    fn color_at(&self, point: Vec3) -> Color;
    fn check_collision(&self, ray: &Ray) -> Option<Vec3>;
    fn normal_at(&self, point: Vec3) -> Option<Vec3>;
    fn effects(&self) -> &TextureEffects;
}

pub struct SceneObject<T: Texture, P: Shape> {
    pub texture: T,
    pub primitive: P,
    pub effects: TextureEffects,
}

impl<T: Texture, P: Shape> AnySceneObject for SceneObject<T, P> {
    fn color_at(&self, point: Vec3) -> Color {
        let (u, v) = self.primitive.surface_mapping_at(point).unwrap();
        self.texture.color_at(u, v)
    }

    fn check_collision(&self, ray: &Ray) -> Option<Vec3> {
        self.primitive.check_collision(ray)
    }

    fn normal_at(&self, point: Vec3) -> Option<Vec3> {
        self.primitive.normal_at(point)
    }

    fn effects(&self) -> &TextureEffects {
        &self.effects
    }
}

pub trait RayEmitter {
    fn generate_rays<'a>(
        &'a self,
        screen_width: u32,
        screen_height: u32,
    ) -> Box<dyn Iterator<Item = (u32, u32, Ray)> + 'a>;
}
