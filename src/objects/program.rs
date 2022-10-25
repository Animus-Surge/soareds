pub mod program {
    use crate::objects::shader::*;
    use std::ffi::{CStr, CString};
    use gl::*;
    use crate::util::util::cstr_whitespace;

    pub struct Program {
        id: gl::types::GLuint
    }

    impl Program {
        pub fn from_shaders(shaders: &[shader::Shader]) -> Result<Program, String> {
            let prog_id = unsafe {gl::CreateProgram()};

            for shader in shaders {
                unsafe {gl::AttachShader(prog_id, shader.id());}
            }

            unsafe {gl::LinkProgram(prog_id)};

            let mut success = 0;
            unsafe {
                gl::GetProgramiv(prog_id, gl::LINK_STATUS, &mut success);
            }

            if success == 0 {
                let mut len: gl::types::GLint = 0;
                unsafe {
                    gl::GetProgramiv(prog_id, gl::INFO_LOG_LENGTH, &mut len);
                }

                let error = cstr_whitespace(len as usize);

                unsafe {
                    gl::GetProgramInfoLog(
                        prog_id,
                        len,
                        std::ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar
                    );
                }

                return Err(error.to_string_lossy().into_owned());
            }

            for shader in shaders {
                unsafe {gl::DetachShader(prog_id, shader.id());}
            }

            Ok(Program {id: prog_id})
        }

        pub fn id(&self) ->gl::types::GLuint {
            self.id
        }
    }

    impl Drop for Program {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteProgram(self.id)
            }
        }
    }

}