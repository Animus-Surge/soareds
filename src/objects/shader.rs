
#[allow(dead_code, unused)]
pub mod shader {
    use std::ffi::{CStr, CString};
    use gl;

    pub struct Shader {
        id: gl::types::GLuint
    }

    impl Shader {
        pub fn from_source(
            source: &CStr,
            kind: gl::types::GLenum
        ) -> Result<Shader, String> {
            let id = load_shader(source, kind)?;
            Ok(Shader { id })
        }

        pub fn id(&self) -> gl::types::GLuint {
            self.id
        }
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe {gl::DeleteShader(self.id);}
        }
    }


    fn load_shader(
        source: &CStr,
        kind: gl::types::GLenum
    ) -> Result<gl::types::GLuint, String> {
        let id = unsafe { gl::CreateShader(kind) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }

        Ok(id)
    }
}