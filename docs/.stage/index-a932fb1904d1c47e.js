var robotlegcanvas, robotlegcontext, robotlegt = 0, loop = true;
var mousecommand = [0,0]

// Commander
const points = [
    [-25, -550],
    [255, -550],
    [255, -500],
    [-25, -400]
]
const distance = points.map((t,i)=>{
    return Math.sqrt( (points[(i+1)%points.length][0]-t[0]) ** 2  + (points[(i+1)%points.length][1]-t[1]) ** 2);
})
const timeweight = [3,1,1,1];

const timetable = distance.map((t,i)=>t * timeweight[i])
const totaltime = timetable.reduce((i,j)=>i+j,0)

console.log(distance)
console.log(timetable)
console.log(totaltime)

const Commander = ()=>{
    robotlegt += 10;
    let t = robotlegt;
    if(t > totaltime) {t = 0; robotlegt = 0;}
    let i = 0;
    while(t > timetable[i]){
        t -= timetable[i]
        i++
    }
    i = (i)%points.length
    return mappoint(t,i);
}

const mappoint = (t,i)=>{
    const p1 = points[i]
    const p2 = points[(i+1)%points.length]
    const T  = timetable[i]

    return [
        p1[0] + t/T*(p2[0] - p1[0]),
        p1[1] + t/T*(p2[1] - p1[1]),
    ]
}

// Inverse kinematic
const A = 200;
const B = 150;
const C = 239.45;
const D = 225;
const F = 460;
const Alpha = 0.757629712;

const IK = (p)=>{
    const x = p[0];
    const y = p[1];
    const r = Math.sqrt(x**2+y**2)
    let Configuration = [
        [0,0],[A,0]
    ]

    const theta1 = x > 0 ?  Math.atan(-y/x) : Math.PI/2
    const theta2 = Math.acos( (B**2 + r**2 - F**2) / 2 / B / r )
    const theta = theta1 + theta2;

    Configuration.push([B * Math.cos(-theta), B * Math.sin(-theta)]);

    let beta = Math.asin( Math.sin(theta2) * r / F );

    if(r > Math.sqrt( F*F + B*B ) && beta < Math.PI) beta = Math.PI - beta;

    const delta = - theta - beta + Alpha + Math.PI;

    const x_e =   B * Math.cos( -theta ) +  C * Math.cos( delta );
    const y_e =   B * Math.sin( -theta ) +  C * Math.sin( delta );

    const r_e = Math.sqrt( (x_e-A)**2 + y_e**2)

    Configuration.push([x_e,y_e])

    let phi = Math.acos( (B**2 + r_e**2 - D**2) / 2 / B / r_e )

    if(x_e < A)  phi += Math.atan(-y_e/(A-x_e));
    if(x_e > A)  phi += Math.atan((x_e-A)/-y_e)+ Math.PI/2;
    if(x_e == A) phi += Math.PI/2

    Configuration.push([B * Math.cos(Math.PI+phi) + A, B * Math.sin(Math.PI+phi)]);

    Configuration.push(p)

    return Configuration
}

// Draw
const scale = [0.23,-0.23]
const offset = [120,10]

const DrawPoint = (p)=>{
    p = [scale[0]*p[0], scale[1]*p[1]];
    p = [offset[0]+p[0], offset[1]+p[1]];
    DrawCircle(p)
}

const DrawCircle = (p,r=2)=>{
    robotlegcontext.beginPath()
    robotlegcontext.arc(p[0], p[1], r, 0, 2 * Math.PI);
    robotlegcontext.fill()
}

const DrawLinkages = (ik)=>{
    DrawLine(ik[0],ik[1])
    DrawLine(ik[0],ik[2])
    DrawLine(ik[2],ik[5])
    DrawLine(ik[2],ik[3])
    DrawLine(ik[5],ik[3])
    DrawLine(ik[1],ik[4])
    DrawLine(ik[3],ik[4])

}

const DrawLine = (a,b)=>{
    robotlegcontext.beginPath()
    robotlegcontext.moveTo(a[0] * scale[0] + offset[0],a[1] * scale[1] + offset[1])
    robotlegcontext.lineTo(b[0] * scale[0] + offset[0],b[1] * scale[1] + offset[1])
    robotlegcontext.stroke()
}


onload = ()=>{
    robotlegcanvas = document.getElementById("robotlegcanvas");
    if (robotlegcanvas == null) {
        console.log("timeout . . .");
        setTimeout(()=>{onload();}, 1000);
        return;
    }

    robotlegcontext = robotlegcanvas.getContext("2d");
    
    robotlegcontext.fillStyle = "#FF0000";

    setInterval(()=>{
        //if(!loop) return
        robotlegcontext.clearRect(0,0,500,500)
        let p = Commander();
        if(!loop) p = mousecommand
        console.log(p)
        ik = IK(p);
        DrawLinkages(ik)
        ik.forEach(p => {
            DrawPoint(p)
        });
    }, 10)

    robotlegcanvas.addEventListener("mouseenter", (event)=>{loop = false})
    robotlegcanvas.addEventListener("mouseleave", (event)=>{loop = true})
    robotlegcanvas.addEventListener("mousemove", (evt)=>{
        var rect = robotlegcanvas.getBoundingClientRect();

        let x = (evt.clientX - rect.left) / (rect.right - rect.left) * robotlegcanvas.width
        let y = (evt.clientY - rect.top) / (rect.bottom - rect.top) * robotlegcanvas.height
        
        mousecommand = [(x - offset[0])/scale[0],(y - offset[1])/scale[1]]

        robotlegcontext.fillRect(x, y, 3, 3);
    })
}

onload();