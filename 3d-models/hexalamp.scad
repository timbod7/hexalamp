num_leds = 30;
led_spacing = 17;
triangle_spacing = led_spacing * cos(30);

wall_thickness = 1.5;
lightbox_depth = 35;
inner_diameter = num_leds * triangle_spacing / PI;
outer_diameter = inner_diameter + lightbox_depth;
layer_height = outer_diameter * PI / num_leds * tan(60);

led_hole_size = 6;
clip_hole_width = 1.5;
clip_hole_length = 4;
clip_hole_offset = 5;
clip_thickness = 1;
clip_length = 1;

module ring(id,od,h) {
  linear_extrude(h) {
    difference() {
      circle(d=od, $fn=200);
      circle(d=id, $fn=200);
    }
  }
}

module led_hole() {


  translate([0,inner_diameter/2,0])
  rotate([0,90,90])
  translate([0,0,-wall_thickness * 3/2])
  linear_extrude(wall_thickness * 4)
  union() {
    square(led_hole_size,true);
    translate([-clip_hole_offset-0.5,0])
    square([clip_hole_width,clip_hole_length],true);
    translate([clip_hole_offset+0.5,0])
    square([clip_hole_width,clip_hole_length],true);
  }
}



module cylinderBetween(p1,p2,d) {
  translate((p1+p2)/2)
  rotate([-acos((p2[2]-p1[2]) / norm(p1-p2)),0,
          -atan2(p2[0]-p1[0],p2[1]-p1[1])])
  cylinder(d=d, h=norm(p1-p2), center = true);
} 

module sequential_hull(){
    for (i = [0: $children-2]) {
        hull(){
            children(i);
            children(i+1);
        }
      }
}

function interp_point(p1, p2, k) = [
   p1[0]*k + p2[0]*(1-k),
   p1[1]*k + p2[1]*(1-k),
   p1[2]*k + p2[2]*(1-k),
];

module divider_seg(p1,p2,p3,p4,t) {
  sequential_hull() {
    cylinderBetween(interp_point(p1,p3,0), interp_point(p2,p4,0), t);
    cylinderBetween(interp_point(p1,p3,0.25), interp_point(p2,p4,0.25), t);
    cylinderBetween(interp_point(p1,p3,0.5), interp_point(p2,p4,0.5), t);
    cylinderBetween(interp_point(p1,p3,0.75), interp_point(p2,p4,0.75), t);
    cylinderBetween(interp_point(p1,p3,1.0), interp_point(p2,p4,1.0), t);
  }
}

function circlexy(r, angle) = [
  r * sin(angle), r * cos(angle)
];

module divider() {
  xy1 = circlexy(inner_diameter/2,0);
  xy2a = circlexy(inner_diameter/2,360/num_leds);
  xy2b = circlexy(inner_diameter/2,-360/num_leds);
  xy3 = circlexy(outer_diameter/2,0);
  xy4a = circlexy(outer_diameter/2,360/num_leds);
  xy4b = circlexy(outer_diameter/2,-360/num_leds);
  p1 = [xy1[0],xy1[1],layer_height];
  p3 = [xy3[0],xy3[1],layer_height];
  p2a = [xy2a[0],xy2a[1],0];
  p4a = [xy4a[0],xy4a[1],0];
  divider_seg(p1, p2a, p3, p4a, wall_thickness);
  
  p2b = [xy2b[0],xy2b[1],0];
  p4b = [xy4b[0],xy4b[1],0];
  divider_seg(p1, p2b, p3, p4b, wall_thickness);
}

module mount_bracket(l) {
  id = 6.5;
  od = 10.5;


  difference() {
    linear_extrude(layer_height)
    difference() {
      union() {
        circle(d=od);
        translate([-od/2,0,0])
        square([od,l]);
      }
      circle(d=id);
    }
  
    h = 5;
    w = layer_height - 4 * wall_thickness;
    translate([od/2,l,layer_height-wall_thickness*4/2])
    rotate([0,90,180])
    linear_extrude(od)
    polygon([
       [0,0],
       [w,0],
       [w-h,h],
       [h,h]
    ]);
  }   
}

module frame() {
  translate([0,0,wall_thickness/2])
  difference() {
    union() {
      ring(inner_diameter-wall_thickness*2, inner_diameter, layer_height);
      translate([0,0,-wall_thickness/2])
      ring(inner_diameter-wall_thickness*2, outer_diameter, wall_thickness);
      intersection() {
        ring(inner_diameter-wall_thickness, outer_diameter, layer_height);
        union() {
          for(i = [0:num_leds/2-1]) {
            rotate([0,0,i * 2 * 360/num_leds])
            divider();
          }
        }
      }
    }
    
    translate([0,0,layer_height-wall_thickness/2])
    ring(inner_diameter-wall_thickness*2, outer_diameter, wall_thickness);
    
    
    la = 360/num_leds;
    for(i = [0:num_leds/2-1]) {
      rotate([0,0,i * la * 2])
      translate([0,0,layer_height/2-led_spacing/2 * sin(30)])
      rotate([0,30,0])
      led_hole();
      
      rotate([0,0,la + i * la * 2])
      translate([0,0,layer_height/2+led_spacing/2 * sin(30)])
      rotate([0,30,0])
      led_hole();
    }
  }
  
  mbl = 10;
  translate([0,inner_diameter/2 -mbl - wall_thickness/2,0])
  mount_bracket(mbl);
  rotate([0,0,180])
  translate([0,inner_diameter/2 -mbl - wall_thickness/2,0])
  mount_bracket(mbl);
}

module pie(a1,a2,r) {
  polygon([
    [0,0],
    [r*sin(a1), r * cos(a1)],
    [r*sin(a2), r * cos(a2)],
  ]);
}

module diffuser() {
  wall_thickness = 1.0;
  overhang_thickness = 1.2;
  num_layers = 6;
  ring(outer_diameter-10, outer_diameter + wall_thickness, overhang_thickness);
  ring(outer_diameter, outer_diameter + wall_thickness, layer_height * num_layers + overhang_thickness); 
}

module led_clip() {
  ct = clip_thickness;
  cho = clip_hole_offset;
  chw = clip_hole_width;
  module arm() {
    polygon([
      [0,0],
      [ct,0],
      [ct,ct + clip_length+wall_thickness+chw+ct],
      [-(chw - ct),ct + clip_length+wall_thickness+ct],
      [-(chw - ct),ct + clip_length+wall_thickness],
      [0,ct + clip_length+wall_thickness],
      [0,clip_length]
    ]);
  }
  
  linear_extrude(clip_hole_length-0.5) {
  translate([-cho-ct,0])
  square([(cho + ct)*2, ct], false);
  translate([cho,0])
  arm();
  translate([-cho,0])
  scale([-1,1])
  arm();
  }
}


//intersection() {
//  translate([0,0,-500])
//  linear_extrude(1000)
//  pie(160, 120, 1000);
//  frame();
//}

// led_clip();
// led_hole();


// frame();

diffuser();

