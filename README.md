# tornament bracket
A small project to create a potential tournament bracket for a beerpong tournament

## Issue on intel integrated graphics
There is a bug that causes sprite rendering not working on intel graphics.
The workaround is to set the backend manualy by running `$env:WGPU_BACKEND="dx12"`
