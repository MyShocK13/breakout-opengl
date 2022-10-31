use std::os::raw::c_void;
use std::ptr;

#[derive(Copy, Clone, Debug)]
pub struct Texture2D {
    // holds the ID of the texture object, used for all texture operations to reference to this particlar texture
    pub id: u32,
    // texture image dimensions in pixels
    pub width: u32,
    pub height: u32,
    // texture Format
    pub internal_format: u32, // format of texture object
    pub image_format: u32, // format of loaded image
    // texture configuration
    pub wrap_s: u32, // wrapping mode on S axis
    pub wrap_t: u32, // wrapping mode on T axis
    pub filter_min: u32, // filtering mode if texture pixels < screen pixels
    pub filter_max: u32, // filtering mode if texture pixels > screen pixels
}

impl Texture2D {
    pub const fn new_empty() -> Self {
        let texture = Texture2D {
            id: 0,
            width: 0,
            height: 0,
            internal_format: gl::RGB,
            image_format: gl::RGB,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            filter_min: gl::LINEAR,
            filter_max: gl::LINEAR,
        };

        texture
    }

    pub fn new() -> Self {
        let mut texture = Texture2D {
            id: 0,
            width: 0,
            height: 0,
            internal_format: gl::RGB,
            image_format: gl::RGB,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            filter_min: gl::LINEAR,
            filter_max: gl::LINEAR,
        };

        unsafe {
            gl::GenTextures(1, &mut texture.id);
        }

        texture
    }

    /// generates texture from image data
    pub unsafe fn generate(&mut self, width: u32, height: u32, data: Vec<u8>) {
        self.width = width;
        self.height = height;
        
        // create Texture
        gl::BindTexture(gl::TEXTURE_2D, self.id);
        gl::TexImage2D(gl::TEXTURE_2D,
                    0,
                    self.internal_format as i32,
                    width as i32,
                    height as i32,
                    0,
                    self.image_format,
                    gl::UNSIGNED_BYTE,
                    &data[0] as *const u8 as *const c_void);
        // set Texture wrap and filter modes
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.filter_min as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.filter_max as i32);

        gl::GenerateMipmap(gl::TEXTURE_2D);
        
        // unbind texture
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    pub unsafe fn generate_raw(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        
        // create Texture
        gl::BindTexture(gl::TEXTURE_2D, self.id);
        gl::TexImage2D(gl::TEXTURE_2D,
                        0,
                        self.internal_format as i32,
                        width as i32,
                        height as i32,
                        0,
                        self.image_format,
                        gl::UNSIGNED_BYTE,
                        ptr::null());
        // set Texture wrap and filter modes
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.filter_min as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.filter_max as i32);

        gl::GenerateMipmap(gl::TEXTURE_2D);
        
        // unbind texture
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    
    /// binds the texture as the current active GL_TEXTURE_2D texture object
    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
}