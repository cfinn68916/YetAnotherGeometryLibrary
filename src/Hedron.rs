use crate::simple_tri::SimpleTriangle;
use crate::vectors::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tetrahedron {
    pub origin:Vector3,
    pub a:Vector3,
    pub b:Vector3,
    pub c:Vector3
}


impl Tetrahedron {
    pub fn new(origin:Vector3 ,a:Vector3, b:Vector3, c:Vector3) -> Self {
        Tetrahedron{ origin,a,b,c}
    }
    pub fn from_points(a:Vector3, b:Vector3, c:Vector3, d:Vector3) -> Self {
        Tetrahedron::new(a,b-a,c-a,d-a)
    }
    pub fn volume(&self) ->f64{
        // This is the scalar triple product, or the determinant of the matrix with a,b,and c as its rows. It is the volume of the parallelepiped with sides a,b,c.
        self.a.dot(&self.b.cross(&self.c))/2.0
    }
    pub fn pt_1(&self)->Vector3{
        self.origin
    }
    pub fn pt_2(&self)->Vector3{
        self.origin+self.a
    }
    pub fn pt_3(&self)->Vector3{
        self.origin+self.b
    }
    pub fn pt_4(&self)->Vector3{
        self.origin+self.c
    }
    pub fn point_in(&self, other:Vector3)->bool{
        let other_adj=other-self.origin;
        let point_in_threespace=Vector3::new(other_adj.dot(&self.a)/self.a.dot(&self.a),other_adj.dot(&self.b)/self.b.dot(&self.b),other_adj.dot(&self.c)/self.c.dot(&self.c));
        (point_in_threespace.x>=0.0)&&(point_in_threespace.y>=0.0)&&(point_in_threespace.z>=0.0)&&(point_in_threespace.dot(&Vector3::new(1.0,1.0,1.0))<=1.0)
    }
    /// Returns the surface triangles. If the volume is positive, the normals of the triangles will point outward, and if it is negative, they will point inward
    ///
    /// returns: [SimpleTriangle;4]
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn surface(&self)->[SimpleTriangle;4]{
        [SimpleTriangle::new(self.pt_1(),self.pt_3(),self.pt_2()),
            SimpleTriangle::new(self.pt_1(),self.pt_4(),self.pt_3()),
            SimpleTriangle::new(self.pt_1(),self.pt_2(),self.pt_4()),
            SimpleTriangle::new(self.pt_2(),self.pt_3(),self.pt_4())]
    }
    pub fn surface_area(&self)->f64{
        let sf=self.surface();
        sf[0].area()+sf[1].area()+sf[2].area()+sf[3].area()
    }
}