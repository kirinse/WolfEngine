#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

//input vertex attributes
layout(location = 0) in vec3	i_pos;
layout(location = 1) in vec2	i_uv;

//input vertex instance attributes
layout (location = 2) in vec3	i_ins_pos;
layout (location = 3) in vec3	i_ins_rot;

layout (binding = 0) uniform UBO_In
{
	mat4 projection_view;
} i_ubo;

layout (binding = 0) uniform UBO 
{
	mat4 projection_view;
} ubo;

//Out
layout (location = 0) out vec2 o_uv;

mat3 rotate_over_axis(float pAngle, vec3 pAxis)
{
	const float a = pAngle;
	const float c = cos(a);
	const float s = sin(a);

	vec3 axis = normalize(pAxis);
	vec3 temp = (1 - c) * axis;

	mat3 rotate;
	rotate[0][0] = c + temp[0] * axis[0];
	rotate[0][1] = temp[0] * axis[1] + s * axis[2];
	rotate[0][2] = temp[0] * axis[2] - s * axis[1];

	rotate[1][0] = temp[1] * axis[0] - s * axis[2];
	rotate[1][1] = c + temp[1] * axis[1];
	rotate[1][2] = temp[1] * axis[2] + s * axis[0];

	rotate[2][0] = temp[2] * axis[0] + s * axis[1];
	rotate[2][1] = temp[2] * axis[1] - s * axis[0];
	rotate[2][2] = c + temp[2] * axis[2];

	return rotate;
}

void main() 
{
    mat3 rx = rotate_over_axis(0.0, vec3( 1.0, 0.0, 0.0));
	mat3 ry = rotate_over_axis(0.0, vec3( 0.0, 1.0, 0.0));
	mat3 rz = rotate_over_axis(0.0, vec3( 0.0, 0.0, 1.0));

	mat3 _rot = rx * ry * rz;
	mat4 _world = mat4( _rot[0][0]			, _rot[0][1]		, _rot[0][2]				, 0.0,
						_rot[1][0]			, _rot[1][1]		, _rot[1][2]				, 0.0,
						_rot[2][0]			, _rot[2][1]		, _rot[2][2]				, 0.0,
						i_ins_pos.x			, i_ins_pos.y		, i_ins_pos.z			    , 1.0);

	vec4 _pos = vec4(i_pos, 1.0);
	gl_Position = i_ubo.projection_view * _world * _pos;

	o_uv = i_uv;
}

