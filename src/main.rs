use regex::*;

#[rustfmt::skip]
mod consts {
    #![allow(unused)]
    pub const FIND_SVG: &str = r"<\s*svg[^>]*>(.*?)<\s*/\s*svg>";
    pub const VALID_COMMAND_CONST: &str = r"/([ml](\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+))))|([hv](\s?-?((\d+(\.\d+)?)|(\.\d+))))|(c(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){5})|(q(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){3}(\s?t?(\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+))))*)|(a(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){2}[,\s]?[01][,\s]+[01][,\s]+([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){2})|(s(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){3})|z/ig";
    pub const IS_VALID_DESCRIPTOR: &str = r"/^m(\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(([ml](\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+))))|([hv](\s?-?((\d+(\.\d+)?)|(\.\d+))))|(c(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){5})|(q(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){3}(\s?t?(\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+))))*)|(a(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){2}[,\s]?[01][,\s]+[01][,\s]+([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){2}))[,\s]?)+z/ig";
    pub const VALID_FLAG_EX: &str = r"/^[01]/";
    pub const COMMA_EX: &str = r"/^(([\t\n\f\r\s]+,?[\t\n\f\r\s]*)|(,[\t\n\f\r\s]*))/";
    pub const VALID_COMMAND_EX: &str = r"/^[\t\n\f\r\s]*([achlmqstvz])[\t\n\f\r\s]*/i";
    pub const VALID_COORDINATE_EX: &str = r"/^[+-]?((\d*\.\d+)|(\d+\.)|(\d+))(e[+-]?\d+)?/i";
    pub const PATH_GRAMMAR: [(&str, &[&str]); 10] = [
        ("z", &[]),
        ("h", &[ VALID_COORDINATE_EX ]),
        ("v", &[ VALID_COORDINATE_EX ]),
        ("m", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX ]),
        ("l", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX ]),
        ("t", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX ]),
        ("s", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ]),
        ("q", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ]),
        ("c", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ]),
        ("a", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_FLAG_EX, VALID_FLAG_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ]),
    ];

    pub static VALID_COMMAND: &str = r"/^[\t\n\f\r\s]*([achlmqstvz])[\t\n\f\r\s]*/i";
    pub static VALID_FLAG: &str = r"/^[01]/";
    pub static VALID_COORDINATE: &str = r"/^[+-]?((\d*\.\d+)|(\d+\.)|(\d+))(e[+-]?\d+)?/i";
    pub static VALID_COMMA: &str = r"/^(([\t\n\f\r\s]+,?[\t\n\f\r\s]*)|(,[\t\n\f\r\s]*))/";
    // static pointGrammar = {
    //     z: () => [],
    //     Z: () => [],
    //     m: ( point, command ) => [ point[ 0 ] + command[ 1 ], point[ 1 ] + command[ 2 ] ],
    //     M: ( point, command ) => command.slice( 1 ),
    //     h: ( point, command ) => [ point[ 0 ] + command[ 1 ], point[ 1 ] ],
    //     H: ( point, command ) => [ command[ 1 ], point[ 1 ] ],
    //     v: ( point, command ) => [ point[ 0 ], point[ 1 ] + command[ 1 ] ],
    //     V: ( point, command ) => [ point[ 0 ], command[ 1 ] ],
    //     l: ( point, command ) => [ point[ 0 ] + command[ 1 ], point[ 1 ] + command[ 2 ] ],
    //     L: ( point, command ) => command.slice( 1 ),
    //     a: ( point, command ) => [ point[ 0 ] + command[ 6 ], point[ 1 ] + command[ 7 ] ],
    //     A: ( point, command ) => command.slice( 6 ),
    //     c: ( point, command ) => [ point[ 0 ] + command[ 5 ], point[ 1 ] + command[ 6 ] ],
    //     C: ( point, command ) => command.slice( 5 ),
    //     t: ( point, command ) => [ point[ 0 ] + command[ 1 ], point[ 1 ] + command[ 2 ] ],
    //     T: ( point, command ) => command.slice( 1 ),
    //     q: ( point, command ) => [ point[ 0 ] + command[ 3 ], point[ 1 ] + command[ 4 ] ],
    //     Q: ( point, command ) => command.slice( 3 ),
    //     s: ( point, command ) => [ point[ 0 ] + command[ 3 ], point[ 1 ] + command[ 4 ] ],
    //     S: ( point, command ) => command.slice( 3 ),
    //     };

}
use consts::*;

fn points( path:&str ) {
   let currentPoint = [ 0, 0 ];
   return parse_raw( path ).reduce( | result, command |  {
      currentPoint = this.pointGrammar[ command[ 0 ] ]( currentPoint, command );
      return [ ...result, currentPoint ];
   }, [] );
}
fn parse_raw( path:&str ) -> Result<[std::ops::RangeFull; 0], &str> {
   let cursor = 0;
   let mut parsedComponents = [];
   while ( cursor < path.len() ) {
      if let Some(matcher) =  path.slice( cursor ).matcher( VALID_COMMAND ) {
         let  command = matcher[ 1 ];
         cursor += matcher[ 0 ].len();
         let componentList =parse_components( command, path, cursor );
         cursor = componentList[ 0 ];
         parsedComponents = [ ...parsedComponents, ...componentList[1] ];
      } else {
return Err(  "Invalid path: first error at char ${ cursor }"  );
      }
   }
   return Ok(parsedComponents);
}
fn parse_components( current_type:char, path:&str, cursor:usize ) {
   let expectedCommands = PATH_GRAMMAR[ current_type.toLowerCase() ];
   let components = [];
   while ( cursor <= path.len() ) {
      let component = [ current_type ];
      for  regexi in expectedCommands  {
         let match = path.slice( cursor ).match( regex );
         if ( match !== null ) {
            component.push( parseInt( match[ 0 ] ) );
            cursor += match[ 0 ].length;
            let nextSlice = path.slice( cursor ).match( this.validComma );
            if ( nextSlice !== null ) cursor += nextSlice[ 0 ].length;
         } else if ( component.length === 1 ) {
            return [ cursor, components ];
         } else {
            throw new Error( "Invalid path: first error at char ${ cursor }" );
         }
      }
      components.push( component );
      if ( expectedCommands.length == 0 ) return [ cursor, components ];
      if ( current_type == 'm' ) current_type = 'l';
      if ( current_type == 'M' ) current_type = 'L';
   }
   throw new Error( "Invalid path: first error at char ${ cursor }" );
}

fn main() {
    println!("Hello, world! {FIND_SVG}");
}


#[test]
fn first() {
    let shape1 = "m 150,150 a 25,25 0 1,1 50,0 a 25,25 0 1,1 -50,0 z";
let shape2 = "m 40 254 s 35 -27 30 -69 s 33 -49 75 -25 z";
let wrongShape = "m l 250 a -400, -350 .";
IS_VALID_DESCRIPTOR.test( shape2 );
//    -> true
IS_VALID_DESCRIPTOR.test( wrongShape );
//    -> false
shape1.match( VALID_COMMAND_CONST );
//    -> [ 'm 150,150', 'a 25,25 0 1,1 50,0', 'a 25,25 0 1,1 -50,0', 'z' ]
shape2.match( VALID_COMMAND_CONST ).map( |command|  command.split( r"/[\s,]/" ).map( |parameter|  parseInt( parameter ) || parameter ) );
//    -> [ [ 'm', 40, 254 ], [ 's', 35, -27, 30, -69 ], [ 's', 33, -49, 75, -25 ], [ 'z' ] ]
}


#[test]
fn second() {
    let shape = "m 150,150 a 25,25 0 1,1 50,0 a 25,25 0 1,1 -50,0 z";
    let parsedShape = parse_raw( shape );
//    -> [ [ 'm', 50, 50 ], [ 'l', 100, 0 ], [ 'l', 0, 100 ], [ 'l', -100, 0 ], [ 'z' ] ]
}
#[test]
fn third() {let sameShape = [
   "m 25,25 l 50,0 l 0,50 l -50,0 z",
   "M 25 25 L 75 25 L 75 75 L 25 75 Z"
];
parse_raw( sameShape[ 0 ] );
//    -> [ [ 25, 25 ], [ 75, 25 ], [ 75, 75 ], [ 25, 75 ], [] ]
parse_raw( sameShape[ 1 ] );
//    -> [ [ 25, 25 ], [ 75, 25 ], [ 75, 75 ], [ 25, 75 ], [] ]
}
// #[test]
// fn fourth() {}