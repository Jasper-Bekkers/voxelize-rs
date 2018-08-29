// fn find_min_max(x0: f32, x1: f32, x2: f32) -> (f32, f32){
//     let mut min = x0;
//     let mut max = x0;

//     if x1 < min { min=x1; }
//     if x1 > max { max=x1; }
//     if x2 < min { min=x2; }
//     if x2 > max { max=x2; }

//     (min, max)
// }

// fn planeBoxOverlap(normal: &Vector3<f32>, vert: &Vector3<f32>, maxbox: &Vector3<f32>) -> bool	// -NJMP-
// {
//     let vmin = Vector3<f32>::Zero();
//     let vmax = Vector3<f32>::Zero();

//     if(normal.x > 0.0)
//     {
//         vmin.x =-maxbox.x - vert.x;	// -NJMP-
//         vmax.x = maxbox.x - vert.x;	// -NJMP-
//     }
//     else
//     {
//         vmin.x = maxbox.x - vert.x;	// -NJMP-
//         vmax.x =-maxbox.x - vert.x;	// -NJMP-
//     }

//     if(normal.y > 0.0)
//     {
//         vmin.y =-maxbox.y - vert.y;	// -NJMP-
//         vmax.y = maxbox.y - vert.y;	// -NJMP-
//     }
//     else
//     {
//         vmin.y = maxbox.y - vert.y;	// -NJMP-
//         vmax.y =-maxbox.y - vert.y;	// -NJMP-
//     }

//     if(normal.z > 0.0)
//     {
//         vmin.z =-maxbox.z - vert.z;	// -NJMP-
//         vmax.z = maxbox.z - vert.z;	// -NJMP-
//     }
//     else
//     {
//         vmin.z = maxbox.z - vert.z;	// -NJMP-
//         vmax.z =-maxbox.z - vert.z;	// -NJMP-
//     }


//     if normal.dot(vmin) > 0.0 { return false; }
//     if normal.dot(vmax) >=0.0 { return true; }

//     false
// }

// /*======================== X-tests ========================
// #define AXISTEST_X01(a, b, fa, fb)			   \
// 	p0 = a*v0.y - b*v0.z;			       	   \
// 	p2 = a*v2.y - b*v2.z;			       	   \
//         if(p0<p2) {min=p0; max=p2;} else {min=p2; max=p0;} \
// 	rad = fa * boxhalfsize.y + fb * boxhalfsize.z;   \
// 	if(min>rad || max<-rad) return 0;
// */
// fn axis_test_x01(a: f32, b: f32, fa: f32, fb: f32, v0: &Vector3<f32>, v2: &Vector3<f32>, boxhalfsize: &Vector3<f32>) -> bool{
//     let p0 = a * v0.y - b * v0.z;
//     let p2 = a * v2.y - b * v2.z;

//     let (min, max) = if p0 < p2 {
//         (p0, p2)
//     }else{
//         (p2, p0)
//     };

//     let rad = fa * boxhalfsize.y + fb * boxhalfsize.z;

//     if (min > rad || max < -rad) {
//         return false;
//     }

//     true
// }


// fn axis_test_x2(a: f32, b: f32, fa: f32, fb: f32, v0: &Vector3<f32>, v1: &Vector3<f32>, boxhalfsize: &Vector3<f32>) -> bool{
//     let p0 = a * v0.y - b * v0.z;
//     let p1 = a * v1.y - b * v1.z;

//     let (min, max) = if p0 < p1 {
//         (p0, p1)
//     }else{
//         (p1, p0)
//     };

//     let rad = fa * boxhalfsize.y + fb * boxhalfsize.z;

//     if (min > rad || max < -rad) {
//         return false;
//     }
    
//     true
// }
// /*
// #define AXISTEST_X2(a, b, fa, fb)			   \
// 	p0 = a*v0.y - b*v0.z;			           \
// 	p1 = a*v1.y - b*v1.z;			       	   \
//         if(p0<p1) {min=p0; max=p1;} else {min=p1; max=p0;} \
// 	rad = fa * boxhalfsize.y + fb * boxhalfsize.z;   \
// 	if(min>rad || max<-rad) return 0;

// /*======================== Y-tests ========================*/
// #define AXISTEST_Y02(a, b, fa, fb)			   \
// 	p0 = -a*v0.x + b*v0.z;		      	   \
// 	p2 = -a*v2.x + b*v2.z;	       	       	   \
//         if(p0<p2) {min=p0; max=p2;} else {min=p2; max=p0;} \
// 	rad = fa * boxhalfsize.x + fb * boxhalfsize.z;   \
// 	if(min>rad || max<-rad) return 0;
// */
// fn axis_test_y02(a: f32, b: f32, fa: f32, fb: f32, v0: &Vector3<f32>, v2: &Vector3<f32>, boxhalfsize: &Vector3<f32>) -> bool{
//     let p0 = -a * v0.x - b * v0.z;
//     let p2 = -a * v2.x - b * v2.z;

//     let (min, max) = if p0 < p2 {
//         (p0, p2)
//     }else{
//         (p2, p0)
//     };

//     let rad = fa * boxhalfsize.x + fb * boxhalfsize.z;

//     if (min > rad || max < -rad) {
//         return false;
//     }
    
//     true
// }
// /*
// #define AXISTEST_Y1(a, b, fa, fb)			   \
// 	p0 = -a*v0.x + b*v0.z;		      	   \
// 	p1 = -a*v1.x + b*v1.z;	     	       	   \
//         if(p0<p1) {min=p0; max=p1;} else {min=p1; max=p0;} \
// 	rad = fa * boxhalfsize.x + fb * boxhalfsize.z;   \
// 	if(min>rad || max<-rad) return 0;
// */
// fn axis_test_y1(a: f32, b: f32, fa: f32, fb: f32, v0: &Vector3<f32>, v1: &Vector3<f32>, boxhalfsize: &Vector3<f32>) -> bool{
//     let p0 = -a * v0.x - b * v0.z;
//     let p1 = -a * v1.x - b * v1.z;

//     let (min, max) = if p0 < p1 {
//         (p0, p1)
//     }else{
//         (p1, p0)
//     };

//     let rad = fa * boxhalfsize.x + fb * boxhalfsize.z;

//     if (min > rad || max < -rad) {
//         return false;
//     }
    
//     true
// }

// /*======================== Z-tests ========================
// #define AXISTEST_Z12(a, b, fa, fb)			   \
// 	p1 = a*v1.x - b*v1.y;			           \
// 	p2 = a*v2.x - b*v2.y;			       	   \
//         if(p2<p1) {min=p2; max=p1;} else {min=p1; max=p2;} \
// 	rad = fa * boxhalfsize.x + fb * boxhalfsize.y;   \
// 	if(min>rad || max<-rad) return 0;
// */
// fn axis_test_z12(a: f32, b: f32, fa: f32, fb: f32, v1: &Vector3<f32>, v2: &Vector3<f32>, boxhalfsize: &Vector3<f32>) -> bool{
//     let p1 = a * v1.x - b * v1.y;
//     let p2 = a * v2.x - b * v2.y;

//     let (min, max) = if p1 < p2 {
//         (p1, p2)
//     }else{
//         (p2, p1)
//     };

//     let rad = fa * boxhalfsize.x + fb * boxhalfsize.y;

//     if (min > rad || max < -rad) {
//         return false;
//     }
    
//     true
// }
// /*
// #define AXISTEST_Z0(a, b, fa, fb)			   \
// 	p0 = a*v0.x - b*v0.y;				   \
// 	p1 = a*v1.x - b*v1.y;			           \
//         if(p0<p1) {min=p0; max=p1;} else {min=p1; max=p0;} \
// 	rad = fa * boxhalfsize.x + fb * boxhalfsize.y;   \
// 	if(min>rad || max<-rad) return 0;
// */
// fn axis_test_x2(a: f32, b: f32, fa: f32, fb: f32, v0: &Vector3<f32>, v1: &Vector3<f32>, boxhalfsize: &Vector3<f32>) -> bool{
//     let p0 = a * v0.x - b * v0.y;
//     let p1 = a * v1.x - b * v1.y;

//     let (min, max) = if p0 < p1 {
//         (p0, p1)
//     }else{
//         (p1, p0)
//     };

//     let rad = fa * boxhalfsize.x + fb * boxhalfsize.y;

//     if (min > rad || max < -rad) {
//         return false;
//     }
    
//     true
// }

// fn triBoxOverlap(boxcenter: &Vector3<f32>, boxhalfsize: &Vector3<f32>, tri: &Triangle) -> bool
// {
//   /*    use separating axis theorem to test overlap between triangle and box */
//   /*    need to test for overlap in these directions: */
//   /*    1) the {x,y,z}-directions (actually, since we use the AABB of the triangle */
//   /*       we do not even need to test these) */
//   /*    2) normal of the triangle */
//   /*    3) crossproduct(edge from tri, {x,y,z}-directin) */
//   /*       this gives 3x3=9 more tests */

//    /* This is the fastest branch on Sun */
//    /* move everything so that the boxcenter is in (0,0,0) */
//    let v0 = tri.p0 - boxcenter;
//    let v1 = tri.p1 - boxcenter;
//    let v2 = tri.p2 - boxcenter;



//    /* compute triangle edges */
//    let e0 = v1 - v0;      /* tri edge 0 */
//    let e1 = v2 - v1;      /* tri edge 1 */
//    let e2 = v0 - v2;      /* tri edge 2 */

//    /* Bullet 3:  */
//    /*  test the 9 tests first (this was faster) */
//    let fex = e0.x.abs();
//    let fey = e0.y.abs();
//    let fez = e0.z.abs();
//    if !axis_test_x01(e0.z, e0.y, fez, fey) { return false; }
//    if !axis_test_y02(e0.z, e0.x, fez, fex) { return false; }
//    if !axis_test_z12(e0.y, e0.x, fey, fex) { return false; }

//    let fex = e1.x.abs();
//    let fey = e1.y.abs();
//    let fez = e1.z.abs();
//    if !axis_test_x01(e1.z, e1.y, fez, fey) { return false; }
//    if !axis_test_y02(e1.z, e1.x, fez, fex) { return false; }
//    if !axis_test_z0(e1.y, e1.x, fey, fex) { return false; }

//    let fex = e2.x.abs();
//    let fey = e2.y.abs();
//    let fez = e2.z.abs();
//    if !axis_test_x2(e2.z, e2.y, fez, fey) { return false; }
//    if !axis_test_y1(e2.z, e2.x, fez, fex) { return false; }
//    if !axis_test_z12(e2.y, e2.x, fey, fex) { return false; }

//    /* Bullet 1: */
//    /*  first test overlap in the {x,y,z}-directions */
//    /*  find min, max of the triangle each direction, and test for overlap in */
//    /*  that direction -- this is equivalent to testing a minimal AABB around */
//    /*  the triangle against the AABB */

//    /* test in X-direction */
//    let (min, max) = find_min_max(v0.x,v1.x,v2.x);
//    if(min>boxhalfsize.x || max<-boxhalfsize.x) return 0;

//    /* test in Y-direction */
//    let (min, max) = find_min_max(v0.y,v1.y,v2.y);
//    if(min>boxhalfsize.y || max<-boxhalfsize.y) return 0;

//    /* test in Z-direction */
//    let (min, max) = find_min_max(v0.z,v1.z,v2.z);
//    if(min>boxhalfsize.z || max<-boxhalfsize.z) return 0;

//    /* Bullet 2: */
//    /*  test if the box intersects the plane of the triangle */
//    /*  compute plane equation of triangle: normal*x+d=0 */
//    let normal = e0.cross(e1);
//    // -NJMP- (line removed here)
//    if !planeBoxOverlap(normal,v0,boxhalfsize){
//         return false;	// -NJMP-
//    } 

//    true   /* box and triangle overlaps */
// }


