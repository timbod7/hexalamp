num_leds = 30;
led_spacing = 17;
triangle_spacing = led_spacing * cos(30);

wall_thickness = 1.5;
lightbox_depth = 35;
inner_diameter = num_leds * triangle_spacing / PI;
outer_diameter = inner_diameter + lightbox_depth;
layer_height = outer_diameter * PI / num_leds * tan(60);
mount_bracket_length = 10;

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
  
  mbl = mount_bracket_length;
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
 
diffuser_overhang_thickness = 1.2;
diffuser_wall_thickness = 1.0;
diffuser_radius_gap = 0.1;
cap_vert_overhang = 5;
cap_horz_overhang = 3;

module diffuser() {
  num_layers = 6;
  od = outer_diameter;
  dot = diffuser_overhang_thickness;
  dd = diffuser_radius_gap * 2;
  total_height = layer_height * num_layers + cap_vert_overhang * 2;
  ring(inner_diameter, od + dd + diffuser_wall_thickness, dot);
  ring(od + dd, od + dd + diffuser_wall_thickness, total_height); 
}

module lower_spacer() {
  ring(inner_diameter, outer_diameter, cap_vert_overhang - diffuser_overhang_thickness); 
}

module top_cap() {
  d = outer_diameter + 2 * (diffuser_radius_gap + diffuser_wall_thickness + cap_horz_overhang);
  difference() {
    linear_extrude(cap_vert_overhang * 2) {
      circle(d=d, $fn=200);
    }
    od1 = outer_diameter + 2 * diffuser_wall_thickness + 4 * diffuser_radius_gap;
    ring(outer_diameter,od1,cap_vert_overhang);
    
    
    mbl = mount_bracket_length;
    translate([0,inner_diameter/2 -mbl - wall_thickness/2,0])
    top_cap_hole();
    rotate([0,0,180])
    translate([0,inner_diameter/2 -mbl - wall_thickness/2,0])
    top_cap_hole();
  }
}

module top_cap_hole() {
   support_thickness = 0.4;
   nut_inset = 2;
   difference() {
     linear_extrude(8) {
        circle(d=6, $fn=200);
     }
     translate([0,0,nut_inset-support_thickness]) {
       linear_extrude(support_thickness) {
          circle(d=6, $fn=200);
       }
     }
   }
   translate([0,0,nut_inset]) {
     linear_extrude(5.5) {
        circle(d=12, $fn=6);
     }
   }

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

mount_bracket(10);
// top_cap_hole();

//intersection() {
//  top_cap();
//  linear_extrude(30) {
//    translate([-10,50])
//    square(size=[20,200]);
//  }
//}


// led_clip();
// frame();
// diffuser();
// lower_spacer();
//top_cap();

