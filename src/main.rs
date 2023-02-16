use regex::*;

mod dict {
    use std::{
        collections::{HashMap, HashSet},
        hash::Hash,
        ops::{Deref, Index, IndexMut},
    };
    pub struct Dict<T, U> {
        hash_map: HashMap<T, U>,
    }
    impl<T, U> Index<T> for Dict<T, U>
    where
        T: Eq + Hash,
    {
        type Output = U;

        fn index(&self, index: T) -> &Self::Output {
            self.hash_map.get(&index).unwrap()
        }
    }
    impl<T, U> IndexMut<T> for Dict<T, U>
    where
        T: Eq + Hash,
    {
        fn index_mut(&mut self, index: T) -> &mut Self::Output {
            self.hash_map.get_mut(&index).unwrap()
        }
    }

    impl<T, U> Dict<T, U> {
        fn new(hash_map: HashMap<T, U>) -> Self {
            Dict { hash_map: hash_map }
        }
    }
}
use dict::*;

#[rustfmt::skip]
mod constas {
    #![allow(unused)]
    use std::collections::HashMap;
    use phf::phf_map;

    use crate::dict::*;
    pub const FIND_SVG: &str = r"<\s*svg[^>]*>(.*?)<\s*/\s*svg>";
    pub const VALID_COMMAND_CONST: &str = r"/([ml](\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+))))|([hv](\s?-?((\d+(\.\d+)?)|(\.\d+))))|(c(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){5})|(q(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){3}(\s?t?(\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+))))*)|(a(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){2}[,\s]?[01][,\s]+[01][,\s]+([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){2})|(s(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){3})|z/ig";
    pub const IS_VALID_DESCRIPTOR: &str = r"/^m(\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(([ml](\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+))))|([hv](\s?-?((\d+(\.\d+)?)|(\.\d+))))|(c(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){5})|(q(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){3}(\s?t?(\s?-?((\d+(\.\d+)?)|(\.\d+)))[,\s]?(-?((\d+(\.\d+)?)|(\.\d+))))*)|(a(\s?-?((\d+(\.\d+)?)|(\.\d+)))([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){2}[,\s]?[01][,\s]+[01][,\s]+([,\s]?(-?((\d+(\.\d+)?)|(\.\d+)))){2}))[,\s]?)+z/ig";
    pub const VALID_FLAG_EX: &str = r"/^[01]/";
    pub const COMMA_EX: &str = r"/^(([\t\n\f\r\s]+,?[\t\n\f\r\s]*)|(,[\t\n\f\r\s]*))/";
    pub const VALID_COMMAND_EX: &str = r"/^[\t\n\f\r\s]*([achlmqstvz])[\t\n\f\r\s]*/i";
    pub const VALID_COORDINATE_EX: &str = r"/^[+-]?((\d*\.\d+)|(\d+\.)|(\d+))(e[+-]?\d+)?/i";
    // pub static PATH_GRAMMAR: Dict<&str, &[&str]> = {
    //     let mut temp = HashMap::<&str, &[&str]>::new();
    //     temp.insert("z", &[]);
    //     temp.insert("h", &[ VALID_COORDINATE_EX ]);
    //     temp.insert("v", &[ VALID_COORDINATE_EX ]);
    //     temp.insert("m", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX ]);
    //     temp.insert("l", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX ]);
    //     temp.insert("t", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX ]);
    //     temp.insert("s", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ]);
    //     temp.insert("q", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ]);
    //     temp.insert("c", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ]);
    //     temp.insert("a", &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_FLAG_EX, VALID_FLAG_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ]);
    //     Dict {
    //         hash_map:temp
    //     }
    // };

    pub const PATH_GRAMMAR: phf::Map<&'static str, &[&str]> = phf_map!  {
        "z"=> &[],
        "h"=> &[ VALID_COORDINATE_EX ],
        "v"=> &[ VALID_COORDINATE_EX ],
        "m"=> &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX ],
        "l"=> &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX ],
        "t"=> &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX ],
        "s"=> &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ],
        "q"=> &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ],
        "c"=> &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ],
        "a"=> &[ VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX, VALID_FLAG_EX, VALID_FLAG_EX, VALID_COORDINATE_EX, VALID_COORDINATE_EX ],
    };

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
use constas::*;

// fn points(path: &str) {
//     let mut currentPoint = [0, 0];
//     return parse_raw(path).unwrap().map(|result, command| {
//         currentPoint = POINT_GRAMMAR[command[0]](currentPoint, command);
//         return [..result, currentPoint];
//     });
// }
// fn parse_raw(path: &str) -> Result<[std::ops::RangeFull; 0], &str> {
//     let cursor = 0;
//     let mut parsedComponents = [];
//     while (cursor < path.len()) {
//         if let Some(matcher) = path[cursor..].matcher(VALID_COMMAND) {
//             let command = matcher[1];
//             cursor += matcher[0].len();
//             let componentList = parse_components(command, path, cursor);
//             cursor = componentList[0];
//             parsedComponents = [..parsedComponents, ..componentList[1]];
//         } else {
//             return Err(&format!("Invalid path: first error at char {cursor}"));
//         }
//     }
//     return Ok(parsedComponents);
// }

// fn parse_components(
//     current_type: char,
//     path: &str,
//     cursor: usize,
// ) -> Result<[std::ops::RangeFull; 0], &str> {
//     let expectedCommands = PATH_GRAMMAR[current_type.toLowerCase()];
//     let components: Vec<&str> = vec![];
//     while (cursor <= path.len()) {
//         let mut component = vec![current_type];
//         for regexi in expectedCommands {
//             let matching = path[cursor..].matcher(regexi);
//             if (matching != null) {
//                 component.push(matching[0]);
//                 cursor += matching[0].len();
//                 let nextSlice = path[cursor..].matcher(VALID_COMMA);
//                 if (nextSlice != null) {
//                     cursor += nextSlice[0].len();
//                 }
//             } else if (component.len() == 1) {
//                 return [cursor, components];
//             } else {
//                 return Err(&format!("Invalid path: first error at char {cursor}"));
//             }
//         }
//         components.push(component);
//         if (expectedCommands.len() == 0) {
//             return [cursor, components];
//         }
//         if (current_type == 'm') {
//             current_type = 'l';
//         }
//         if (current_type == 'M') {
//             current_type = 'L';
//         }
//     }
//     return Err(&format!("Invalid path: first error at char {cursor}"));
// }

#[allow(unused)]
const TEXT1: &str = r##"<div class="chartdiv" style="width: 100%; height: 300px; overflow: hidden; text-align: left;" id="chartdiv-projection-projekcia"><div class="amcharts-main-div" style="position: relative; width: 100%; height: 100%;"><div class="amcharts-chart-div" style="overflow: hidden; position: absolute; text-align: left; width: 489.125px; height: 300px; padding: 0px; left: 0px;"><svg version="1.1" style="position: absolute; width: 489.125px; height: 300px; top: 0.1875px; left: 0px;"><desc>JavaScript chart by amCharts 3.21.2</desc><g><path cs="100,100" d="M0.5,0.5 L488.5,0.5 L488.5,299.5 L0.5,299.5 Z" fill="#FFFFFF" stroke="#000000" fill-opacity="0" stroke-width="1" stroke-opacity="0"></path><path cs="100,100" d="M0.5,0.5 L354.5,0.5 L354.5,218.5 L0.5,218.5 L0.5,0.5 Z" fill="#FFFFFF" stroke="#000000" fill-opacity="0" stroke-width="1" stroke-opacity="0" transform="translate(114,20)"></path></g><g><g transform="translate(114,20)" visibility="visible"><g><path cs="100,100" d="M0.5,218.5 L6.5,218.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(-6,0)"></path><path cs="100,100" d="M0.5,218.5 L0.5,218.5 L354.5,218.5" fill="none" stroke-width="1" stroke-opacity="0.1" stroke="#000000"></path></g><g><path cs="100,100" d="M0.5,164.5 L6.5,164.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(-6,0)"></path><path cs="100,100" d="M0.5,164.5 L0.5,164.5 L354.5,164.5" fill="none" stroke-width="1" stroke-opacity="0.1" stroke="#000000"></path></g><g><path cs="100,100" d="M0.5,109.5 L6.5,109.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(-6,0)"></path><path cs="100,100" d="M0.5,109.5 L0.5,109.5 L354.5,109.5" fill="none" stroke-width="1" stroke-opacity="0.1" stroke="#000000"></path></g><g><path cs="100,100" d="M0.5,55.5 L6.5,55.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(-6,0)"></path><path cs="100,100" d="M0.5,55.5 L0.5,55.5 L354.5,55.5" fill="none" stroke-width="1" stroke-opacity="0.1" stroke="#000000"></path></g><g><path cs="100,100" d="M0.5,0.5 L6.5,0.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(-6,0)"></path><path cs="100,100" d="M0.5,0.5 L0.5,0.5 L354.5,0.5" fill="none" stroke-width="1" stroke-opacity="0.1" stroke="#000000"></path></g></g><g transform="translate(114,20)" visibility="visible"><g><path cs="100,100" d="M0.5,0.5 L0.5,5.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(0,218)"></path><path cs="100,100" d="M0.5,218.5 L0.5,218.5 L0.5,0.5" fill="none" stroke-width="1" stroke-opacity="0.1" stroke="#000000"></path></g><g><path cs="100,100" d="M118.5,0.5 L118.5,5.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(0,218)"></path><path cs="100,100" d="M118.5,218.5 L118.5,218.5 L118.5,0.5" fill="none" stroke-width="1" stroke-opacity="0.1" stroke="#000000"></path></g><g><path cs="100,100" d="M236.5,0.5 L236.5,5.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(0,218)"></path><path cs="100,100" d="M236.5,218.5 L236.5,218.5 L236.5,0.5" fill="none" stroke-width="1" stroke-opacity="0.1" stroke="#000000"></path></g><g><path cs="100,100" d="M354.5,0.5 L354.5,5.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(0,218)"></path><path cs="100,100" d="M354.5,218.5 L354.5,218.5 L354.5,0.5" fill="none" stroke-width="1" stroke-opacity="0.1" stroke="#000000"></path></g></g></g><g></g><g></g><g></g><g></g><g clip-path="url(#AmChartsEl-4)"><g transform="translate(114,20)"><g></g><g><path cs="100,100" d="M0.5,218.5 L12.3,217.29141 L24.1,215.93778 L35.9,214.42173 L47.7,212.72374 L59.5,210.822 L71.3,208.69205 L83.1,206.3065 L94.9,203.63469 L106.7,200.64226 L118.5,197.29074 L130.3,193.53704 L142.1,189.33289 L153.9,184.62424 L165.7,179.35056 L177.5,173.44404 L189.3,166.82873 L201.1,159.41958 L212.9,151.12134 L224.7,141.82731 L236.5,131.41799 L248.3,119.75956 L260.1,106.70212 L271.9,92.07778 L283.7,75.69852 L295.5,57.35375 M0,0 L0,0" fill="none" stroke-width="2" stroke-opacity="0.9" stroke="#6840b4" stroke-linejoin="round"></path></g><g></g></g><g transform="translate(114,20)"><g></g><g><path cs="100,100" d="M0.5,218.5 L12.3,217.4209 L24.1,216.3418 L35.9,215.2627 L47.7,214.1836 L59.5,213.1045 L71.3,212.0254 L83.1,210.9463 L94.9,209.8672 L106.7,208.7881 L118.5,207.709 L130.3,206.6299 L142.1,205.5508 L153.9,204.4717 L165.7,203.3926 L177.5,202.3135 L189.3,201.2344 L201.1,200.1553 L212.9,199.0762 L224.7,197.9971 L236.5,196.918 L248.3,195.8389 L260.1,194.7598 L271.9,193.6807 L283.7,192.6016 L295.5,191.5225 M0,0 L0,0" fill="none" stroke-width="2" stroke-opacity="0.9" stroke="#d5bee8" stroke-linejoin="round"></path></g><g></g></g></g><g clip-path="url(#AmChartsEl-6)"></g><g><path cs="100,100" d="M0.5,218.5 L0.5,0.5 L0.5,0.5" fill="none" stroke-width="1" stroke-opacity="0.2" stroke="#000000" transform="translate(114,20)"></path><path cs="100,100" d="M0.5,218.5 L354.5,218.5 L354.5,218.5" fill="none" stroke-width="1" stroke-opacity="0.2" stroke="#000000" transform="translate(114,20)"></path><g><path cs="100,100" d="M0.5,0.5 L0.5,218.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(114,20)" visibility="visible"></path></g><g><path cs="100,100" d="M0.5,0.5 L354.5,0.5" fill="none" stroke-width="1" stroke-opacity="100" stroke="#000000" transform="translate(114,238)" visibility="visible"></path></g></g><g></g><g></g><g clip-path="url(#AmChartsEl-5)"><g transform="translate(114,20)"><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(0,218)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(12,217)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(24,215)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(35,214)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(47,212)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(59,210)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(71,208)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(83,206)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(94,203)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(106,200)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(118,197)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(130,193)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(142,189)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(153,184)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(165,179)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(177,173)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(189,166)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(201,159)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(212,151)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(224,141)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(236,131)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(248,119)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(260,106)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(271,92)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(283,75)" aria-label="Account Balance  "></circle><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#6840b4" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(295,57)" aria-label="Account Balance  "></circle></g><g transform="translate(114,20)"><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(0,218)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(12,217)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(24,216)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(35,215)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(47,214)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(59,213)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(71,212)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(83,210)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(94,209)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(106,208)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(118,207)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(130,206)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(142,205)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(153,204)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(165,203)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(177,202)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(189,201)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(201,200)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(212,199)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(224,197)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(236,196)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(248,195)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(260,194)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(271,193)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(283,192)" aria-label="Cumulative Contribution  "></circle><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#d5bee8" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(295,191)" aria-label="Cumulative Contribution  "></circle></g></g><g><g></g></g><g><g transform="translate(114,20)" visibility="visible"><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="end" transform="translate(-12,217)"><tspan y="6" x="0">0</tspan></text><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="end" transform="translate(-12,163)"><tspan y="6" x="0">5,000,000</tspan></text><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="end" transform="translate(-12,108)"><tspan y="6" x="0">10,000,000</tspan></text><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="end" transform="translate(-12,54)"><tspan y="6" x="0">15,000,000</tspan></text><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="end" transform="translate(-12,-1)"><tspan y="6" x="0">20,000,000</tspan></text><text y="6" fill="#000000" font-family="Verdana" font-size="12px" opacity="1" font-weight="bold" text-anchor="middle" transform="translate(-93,109) rotate(-90)"><tspan y="6" x="0">Value in ?</tspan></text></g><g transform="translate(114,20)" visibility="visible"><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="middle" transform="translate(0,235.5)"><tspan y="6" x="0">0</tspan></text><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="middle" transform="translate(118,235.5)"><tspan y="6" x="0">10</tspan></text><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="middle" transform="translate(236,235.5)"><tspan y="6" x="0">20</tspan></text><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="middle" transform="translate(354,235.5)"><tspan y="6" x="0">30</tspan></text><text y="6" fill="#000000" font-family="Verdana" font-size="12px" opacity="1" font-weight="bold" text-anchor="middle" transform="translate(177,261)"><tspan y="6" x="0">Duration in years</tspan></text></g></g><g></g><g transform="translate(114,20)"></g><g></g><g></g><clipPath id="AmChartsEl-4"><rect x="114" y="20" width="355" height="219" rx="0" ry="0" stroke-width="0"></rect></clipPath><clipPath id="AmChartsEl-5"><rect x="114" y="20" width="355" height="219" rx="0" ry="0" stroke-width="0"></rect></clipPath><clipPath id="AmChartsEl-6"><rect x="114" y="20" width="355" height="219" rx="0" ry="0" stroke-width="0"></rect></clipPath></svg></div><div class="amChartsLegend amcharts-legend-div" style="overflow: hidden; position: relative; text-align: left; width: 190.875px; height: 66px; left: 489.125px; top: 117px;"><svg version="1.1" style="position: absolute; width: 245.875px; height: 66px;"><desc>JavaScript chart by amCharts 3.21.2</desc><g transform="translate(10,0)"><path cs="100,100" d="M0.5,0.5 L171.5,0.5 L171.5,65.5 L0.5,65.5 Z" fill="#FFFFFF" stroke="#000000" fill-opacity="0" stroke-width="1" stroke-opacity="0"></path><g transform="translate(0,11)"><g cursor="pointer" aria-label="Account Balance" transform="translate(0,0)"><g><path cs="100,100" d="M0.5,8.5 L32.5,8.5" fill="none" stroke-width="2" stroke-opacity="0.9" stroke="#6840b4"></path><circle r="4" cx="0" cy="0" fill="#6840b4" stroke="#000000" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(17,8)"></circle></g><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="start" transform="translate(37,7)"><tspan y="6" x="0">Account Balance</tspan></text><rect x="32" y="0" width="134.375" height="18" rx="0" ry="0" stroke-width="0" stroke="none" fill="#fff" fill-opacity="0.005"></rect></g><g cursor="pointer" aria-label="Cumulative Contribution" transform="translate(0,28)"><g><path cs="100,100" d="M0.5,8.5 L32.5,8.5" fill="none" stroke-width="2" stroke-opacity="0.9" stroke="#d5bee8"></path><circle r="4" cx="0" cy="0" fill="#d5bee8" stroke="#000000" fill-opacity="1" stroke-width="2" stroke-opacity="0" transform="translate(17,8)"></circle></g><text y="6" fill="#000000" font-family="Verdana" font-size="11px" opacity="1" text-anchor="start" transform="translate(37,7)"><tspan y="6" x="0">Cumulative Contribution</tspan></text><rect x="32" y="0" width="134.375" height="18" rx="0" ry="0" stroke-width="0" stroke="none" fill="#fff" fill-opacity="0.005"></rect></g></g></g></svg></div></div></div>"##;

// #[rustfmt::skip]
mod consts {
    #![allow(unused)]

    use regex::Captures;
    pub const REGEX_SVG_NUM_POINT: &str = r"(?P<num>-?(?P<unit>\d{1,})?(?:.?(?P<deci>\d+)?))";
    pub const REGEX_SVG_NUM_COMA: &str = r"(?P<num>-?(?P<unit>\d{1,})?(?:,?(?P<deci>\d+)?))";

    pub const MARKER_REG_EX: &str = r"[MmLlSsQqLlHhVvCcSsQqTtAaZz]";
    pub const DIGIT_REG_EX: &str = r"-?[0-9]*\.?\d*";
    pub const SVG_PATH_OPS: &str =
        r"(?P<marker>[MmLlSsQqLlHhVvCcSsQqTtAaZz])|(?P<pos>-?[0-9]*\.?\d*[ ,]-?[0-9]*\.?\d*)";

    fn temp() {
        let taze = r"(?x)  
    (?P<x>
        -?
        [0-9]*
        \.?
        \d*
    )";
        let tazeaz = r"(?x)  
    (?P<num>
        -?
        (?P<unit>
            \d{1,}
        )?
        (?:
            .?
            (?P<deci>
                \d+
            )?
        )
    )
    ";
        let tazeaz = r"(?x)  
    (?P<num>
        -?(?:\d+)|(?:\d+\.)|(?:\d+\.\d+)
        (?:
            .?
            (?P<deci>
                \d+
            )?
        )
    )
    ";
    }
}
#[allow(unused)]
use consts::*;

#[deprecated]
pub fn matcher(capture: &Captures) -> Vec<bool> {
    let mut temp = vec![];
    for element in capture.iter() {
        temp.push(match element {
            Some(_) => true,
            None => false,
        })
    }
    // debug_assert!(temp[0]); // seems to be always the case but might be wrong to be put here
    temp
}
/// Parser for SVG paths
///  # Example
/// ```rust
/// # fn main() {
/// let all = vec![
///     "",
///     "M-11.11,-22 L.33-44  ac55    66 h77  M88 .99  Z",
///     "M500,500 L500,200 L800,500 z M400,600 L400,900 L100,600 z",
///     "M70.491,50.826c-2.232,1.152-6.913,2.304-12.817,2.304c-13.682,0-23.906-8.641-23.906-24.626c0-15.266,10.297-25.49,25.346-25.49c5.977,0,9.865,1.296,11.521,2.16l-1.584,5.112C66.747,9.134,63.363,8.27,59.33,8.27c-11.377,0-18.938,7.272-18.938,20.018c0,11.953,6.841,19.514,18.578,19.514c3.888,0,7.777-0.792,10.297-2.016L70.491,50.826z",
///     "M10,10",
///     "Z",
///     "m 150,150 a 25,25 0 1,1 50,0 a 25,25 0 1,1 -50,0 z",
///     "m 40 254 s 35 -27 30 -69 s 33 -49 75 -25 z",
///     "m 40. 254.5  z",
///     "m l 250 a -400, -350 .",
///     "M10,10 l 5,7 C-5,7.2,.3-16,24,10  z"
/// ];
/// for a in all {
///     println!("");
///     println!("{a:?}");
///     println!("{:?}", list_ops(a));
///     for e in list_ops(a) {
///         println!("{}", e);
///     }
///     println!("{:?}", prettier_ops(a));
/// }
/// #}
/// ```
mod svg_path {
    // #![allow(unused)]

    // MoveTo *[Mm] *, *(?:x *, *y)+ *
    // ClosePath *[Zz] *
    // LineTo *[Ll] *, *(?:x *, *y)+ *
    // HorizontalLineTo *[Hh] *, *x+ *
    // VerticalLineTo *[Vv] *, *y+ *
    // CurveTo *[Cc] *, *(?:x1 *, *y1 *, *x2 *, *y2 *, *x *, *y)+ *
    // ShorthandCurveTo *[Ss] *, *(?:x2 *, *y2 *, *x *, *y)+ *
    // QuadraticBézierCurveTo *[Qq] *, *(?:x1 *, *y1 *, *x *, *y)+ *
    // ShorthandQuadraticBézierCurveTo *[Tt] *, *(?:x *, *y)+ *
    // EllipticalArc *[Aa] *, *(?:rx *, *ry *, *x-axis-rotation *, *large-arc-flag *, *sweep-flag *, *x *, *y)+ *

    use std::fmt::Display;

    use regex::{Captures, Regex, RegexSet};

    pub const MOVE_TO: &str = r" *(?P<marker>[Mm])[,\s]*(?:(?P<x>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.)))+ *";
    pub const CLOSE_PATH: &str = r" *(?P<marker>[Zz]) *";
    pub const LINE_TO: &str = r" *(?P<marker>[Ll])[,\s]*(?:(?P<x>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.)))+ *";
    pub const HORIZONTAL_LINE_TO: &str =
        r" *(?P<marker>[Hh])[,\s]*(?P<x>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))+ *";
    pub const VERTICAL_LINE_TO: &str =
        r" *(?P<marker>[Vv])[,\s]*(?P<y>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))+ *";
    pub const CURVE_TO: &str = r" *(?P<marker>[Cc])[,\s]*(?:(?P<x1>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y1>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<x2>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y2>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<x>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.)))+ *";
    pub const SHORTHAND_CURVE_TO: &str = r" *(?P<marker>[Ss])[,\s]*(?:(?P<x2>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y2>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<x>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.)))+ *";
    pub const QUADRATIC_BÉZIER_CURVE_TO: &str = r" *(?P<marker>[Qq])[,\s]*(?:(?P<x1>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y1>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<x>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.)))+ *";
    pub const SHORTHAND_QUADRATIC_BÉZIER_CURVE_TO: &str = r" *(?P<marker>[Tt])[,\s]*(?:(?P<x>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.)))+ *";
    pub const ELLIPTICAL_ARC: &str = r" *(?P<marker>[Aa])[,\s]*(?:(?P<rx>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<ry>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<x_axis_rotation>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]+(?P<large_arc_flag>[01])[,\s]+(?P<sweep_flag>[01])[,\s]*(?P<x>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.))[,\s]*(?P<y>-?(?:[0-9]*\.?\d+\.?)|(?:[0-9]*\.)))+ *";

    pub const LIST_REGEX_PATH_SVG: [(PathOps<Option<&'static str>>, &'static str); 10] = [
        (PathOps::MoveTo { x: None, y: None }, MOVE_TO),
        (PathOps::ClosePath, CLOSE_PATH),
        (PathOps::LineTo { x: None, y: None }, LINE_TO),
        (PathOps::HorizontalLineTo { x: None }, HORIZONTAL_LINE_TO),
        (PathOps::VerticalLineTo { y: None }, VERTICAL_LINE_TO),
        (
            PathOps::CurveTo {
                x1: None,
                y1: None,
                x2: None,
                y2: None,
                x: None,
                y: None,
            },
            CURVE_TO,
        ),
        (
            PathOps::ShorthandCurveTo {
                x2: None,
                y2: None,
                x: None,
                y: None,
            },
            SHORTHAND_CURVE_TO,
        ),
        (
            PathOps::QuadraticBézierCurveTo {
                x1: None,
                y1: None,
                x: None,
                y: None,
            },
            QUADRATIC_BÉZIER_CURVE_TO,
        ),
        (
            PathOps::ShorthandQuadraticBézierCurveTo { x: None, y: None },
            SHORTHAND_QUADRATIC_BÉZIER_CURVE_TO,
        ),
        (
            PathOps::EllipticalArc {
                rx: None,
                ry: None,
                x_axis_rotation: None,
                large_arc_flag: None,
                sweep_flag: None,
                x: None,
                y: None,
            },
            ELLIPTICAL_ARC,
        ),
    ];

    #[derive(Debug)]
    pub enum PathOps<T> {
        MoveTo {
            x: T,
            y: T,
        },

        ClosePath,

        LineTo {
            x: T,
            y: T,
        },

        HorizontalLineTo {
            x: T,
        },

        VerticalLineTo {
            y: T,
        },

        CurveTo {
            x1: T,
            y1: T,
            x2: T,
            y2: T,
            x: T,
            y: T,
        },

        ShorthandCurveTo {
            x2: T,
            y2: T,
            x: T,
            y: T,
        },

        QuadraticBézierCurveTo {
            x1: T,
            y1: T,
            x: T,
            y: T,
        },

        ShorthandQuadraticBézierCurveTo {
            x: T,
            y: T,
        },

        EllipticalArc {
            rx: T,
            ry: T,
            x_axis_rotation: T,
            large_arc_flag: T,
            sweep_flag: T,
            x: T,
            y: T,
        },
    }

    #[derive(Debug)]
    pub struct ResultOpsPathSvg {
        relative: bool,
        start: usize,
        #[allow(unused)]
        end: usize,
        carac: PathOps<String>,
    }
    #[derive(Debug)]
    pub enum SVGPathOps {
        Relative(PathOps<f64>),
        Absolute(PathOps<f64>),
    }

    impl Display for SVGPathOps {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut to_write = String::new();
            let ops;
            let is_relative;
            match self {
                SVGPathOps::Relative(relative_ops) => (is_relative, ops) = (true, relative_ops),
                SVGPathOps::Absolute(absolute_ops) => (is_relative, ops) = (false, absolute_ops),
            }
            let hr = |c: char| -> char {
                // hr: handle_relative
                if is_relative {
                    c.to_lowercase()
                        .to_string()
                        .char_indices()
                        .next()
                        .unwrap()
                        .1
                } else {
                    c
                }
            };
            match ops {
                PathOps::MoveTo { x, y } => {
                    to_write.push_str(format!("{} {},{}", hr('M'), x, y).as_str())
                }
                PathOps::ClosePath => to_write.push_str(format!("{}", hr('Z')).as_str()),
                PathOps::LineTo { x, y } => {
                    to_write.push_str(format!("{} {},{}", hr('L'), x, y).as_str())
                }
                PathOps::HorizontalLineTo { x } => {
                    to_write.push_str(format!("{} {}", hr('H'), x).as_str())
                }
                PathOps::VerticalLineTo { y } => {
                    to_write.push_str(format!("{} {}", hr('V'), y).as_str())
                }
                PathOps::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => to_write.push_str(
                    format!("{} {},{} {},{} {},{}", hr('C'), x1, y1, x2, y2, x, y).as_str(),
                ),
                PathOps::ShorthandCurveTo { x2, y2, x, y } => {
                    to_write.push_str(format!("{} {},{} {},{}", hr('S'), x2, y2, x, y).as_str())
                }
                PathOps::QuadraticBézierCurveTo { x1, y1, x, y } => {
                    to_write.push_str(format!("{} {},{} {},{}", hr('Q'), x1, y1, x, y).as_str())
                }
                PathOps::ShorthandQuadraticBézierCurveTo { x, y } => {
                    to_write.push_str(format!("{} {},{}", hr('T'), x, y).as_str())
                }
                PathOps::EllipticalArc {
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc_flag,
                    sweep_flag,
                    x,
                    y,
                } => to_write.push_str(
                    format!(
                        "{} {},{} {} {},{} {},{}",
                        hr('A'),
                        rx,
                        ry,
                        x_axis_rotation,
                        large_arc_flag,
                        sweep_flag,
                        x,
                        y
                    )
                    .as_str(),
                ),
            };
            write!(f, "{}", to_write)
        }
    }

    impl Into<SVGPathOps> for ResultOpsPathSvg {
        fn into(self) -> SVGPathOps {
            let ops = match self.carac {
                PathOps::MoveTo { x, y } => PathOps::MoveTo {
                    x: x.parse::<f64>().unwrap(),
                    y: y.parse::<f64>().unwrap(),
                },
                PathOps::ClosePath => PathOps::ClosePath,
                PathOps::LineTo { x, y } => PathOps::LineTo {
                    x: x.parse::<f64>().unwrap(),
                    y: y.parse::<f64>().unwrap(),
                },
                PathOps::HorizontalLineTo { x } => PathOps::HorizontalLineTo {
                    x: x.parse::<f64>().unwrap(),
                },
                PathOps::VerticalLineTo { y } => PathOps::VerticalLineTo {
                    y: y.parse::<f64>().unwrap(),
                },
                PathOps::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => PathOps::CurveTo {
                    x1: x1.parse::<f64>().unwrap(),
                    y1: y1.parse::<f64>().unwrap(),
                    x2: x2.parse::<f64>().unwrap(),
                    y2: y2.parse::<f64>().unwrap(),
                    x: x.parse::<f64>().unwrap(),
                    y: y.parse::<f64>().unwrap(),
                },
                PathOps::ShorthandCurveTo { x2, y2, x, y } => PathOps::ShorthandCurveTo {
                    x2: x2.parse::<f64>().unwrap(),
                    y2: y2.parse::<f64>().unwrap(),
                    x: x.parse::<f64>().unwrap(),
                    y: y.parse::<f64>().unwrap(),
                },
                PathOps::QuadraticBézierCurveTo { x1, y1, x, y } => {
                    PathOps::QuadraticBézierCurveTo {
                        x1: x1.parse::<f64>().unwrap(),
                        y1: y1.parse::<f64>().unwrap(),
                        x: x.parse::<f64>().unwrap(),
                        y: y.parse::<f64>().unwrap(),
                    }
                }
                PathOps::ShorthandQuadraticBézierCurveTo { x, y } => {
                    PathOps::ShorthandQuadraticBézierCurveTo {
                        x: x.parse::<f64>().unwrap(),
                        y: y.parse::<f64>().unwrap(),
                    }
                }
                PathOps::EllipticalArc {
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc_flag,
                    sweep_flag,
                    x,
                    y,
                } => PathOps::EllipticalArc {
                    rx: rx.parse::<f64>().unwrap(),
                    ry: ry.parse::<f64>().unwrap(),
                    x_axis_rotation: x_axis_rotation.parse::<f64>().unwrap(),
                    large_arc_flag: large_arc_flag.parse::<f64>().unwrap(),
                    sweep_flag: sweep_flag.parse::<f64>().unwrap(),
                    x: x.parse::<f64>().unwrap(),
                    y: y.parse::<f64>().unwrap(),
                },
            };
            match self.relative {
                true => SVGPathOps::Relative(ops),
                false => SVGPathOps::Absolute(ops),
            }
        }
    }

    pub fn list_ops(input: &str) -> Vec<SVGPathOps> {
        let var = list_ops_path_svg(input);
        let mut temp = vec![];
        for e in var {
            temp.push(e.into())
        }
        temp
    }

    pub fn prettier_ops(input: &str) -> String {
        list_ops(input)
            .iter()
            .map(|some| format!("{some}"))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn list_ops_path_svg(input: &str) -> Vec<ResultOpsPathSvg> {
        let mut temp_buf = vec![];
        for e in LIST_REGEX_PATH_SVG {
            let regex = Regex::new(e.1).unwrap();
            // println!("{:#?}:{}", regex.is_match(input), e.1);
            for f in regex.captures_iter(input) {
                // println!("{:?}", f);
                // for g in (f.iter(), f.name("x")) {
                //     println!("{:?}", g);
                // }
                #[inline]
                fn get_value(cap: &Captures, name: &str) -> String {
                    let matcher = cap.name(name).unwrap();
                    // println!("{:?}{:?}", matcher, matcher.as_str());
                    matcher.as_str().to_string()
                }
                temp_buf.push(ResultOpsPathSvg {
                    relative: f
                        .name("marker")
                        .unwrap()
                        .as_str()
                        .to_string()
                        .chars()
                        .all(|char| char.is_lowercase()),
                    start: f.get(0).unwrap().start(),
                    end: f.get(0).unwrap().end(),
                    carac: match e.0 {
                        PathOps::MoveTo { .. } => PathOps::MoveTo {
                            x: get_value(&f, "x"),
                            y: get_value(&f, "y"),
                        },
                        PathOps::ClosePath => PathOps::ClosePath,
                        PathOps::LineTo { .. } => PathOps::LineTo {
                            x: get_value(&f, "x"),
                            y: get_value(&f, "y"),
                        },
                        PathOps::HorizontalLineTo { .. } => PathOps::HorizontalLineTo {
                            x: get_value(&f, "x"),
                        },
                        PathOps::VerticalLineTo { .. } => PathOps::VerticalLineTo {
                            y: get_value(&f, "y"),
                        },
                        PathOps::CurveTo { .. } => PathOps::CurveTo {
                            x1: get_value(&f, "x1"),
                            y1: get_value(&f, "y1"),
                            x2: get_value(&f, "x2"),
                            y2: get_value(&f, "y2"),
                            x: get_value(&f, "x"),
                            y: get_value(&f, "y"),
                        },
                        PathOps::ShorthandCurveTo { .. } => PathOps::ShorthandCurveTo {
                            x2: get_value(&f, "x2"),
                            y2: get_value(&f, "y2"),
                            x: get_value(&f, "x"),
                            y: get_value(&f, "y"),
                        },
                        PathOps::QuadraticBézierCurveTo { .. } => {
                            PathOps::QuadraticBézierCurveTo {
                                x1: get_value(&f, "x1"),
                                y1: get_value(&f, "y1"),
                                x: get_value(&f, "x"),
                                y: get_value(&f, "y"),
                            }
                        }
                        PathOps::ShorthandQuadraticBézierCurveTo { .. } => {
                            PathOps::ShorthandQuadraticBézierCurveTo {
                                x: get_value(&f, "x"),
                                y: get_value(&f, "y"),
                            }
                        }
                        PathOps::EllipticalArc { .. } => PathOps::EllipticalArc {
                            rx: get_value(&f, "rx"),
                            ry: get_value(&f, "ry"),
                            x_axis_rotation: get_value(&f, "x_axis_rotation"),
                            large_arc_flag: get_value(&f, "large_arc_flag"),
                            sweep_flag: get_value(&f, "sweep_flag"),
                            x: get_value(&f, "x"),
                            y: get_value(&f, "y"),
                        },
                    },
                });
            }
        }
        temp_buf.sort_by_key(|k| k.start);
        temp_buf
    }

    #[allow(unused)]
    #[deprecated]
    pub fn carac(path: &str) -> Vec<PathOps<String>> {
        let set = RegexSet::new(&[
            MOVE_TO,
            CLOSE_PATH,
            LINE_TO,
            HORIZONTAL_LINE_TO,
            VERTICAL_LINE_TO,
            CURVE_TO,
            SHORTHAND_CURVE_TO,
            QUADRATIC_BÉZIER_CURVE_TO,
            SHORTHAND_QUADRATIC_BÉZIER_CURVE_TO,
            ELLIPTICAL_ARC,
        ])
        .unwrap();

        let matches: Vec<PathOps<String>> = set
            .matches(path)
            .into_iter()
            .map(|match_idx| match set.patterns()[match_idx].as_str() {
                MOVE_TO => PathOps::MoveTo {
                    x: String::new(),
                    y: String::new(),
                },
                CLOSE_PATH => PathOps::ClosePath,
                LINE_TO => PathOps::LineTo {
                    x: String::new(),
                    y: String::new(),
                },
                HORIZONTAL_LINE_TO => PathOps::HorizontalLineTo { x: String::new() },
                VERTICAL_LINE_TO => PathOps::VerticalLineTo { y: String::new() },
                CURVE_TO => PathOps::CurveTo {
                    x1: String::new(),
                    y1: String::new(),
                    x2: String::new(),
                    y2: String::new(),
                    x: String::new(),
                    y: String::new(),
                },
                SHORTHAND_CURVE_TO => PathOps::ShorthandCurveTo {
                    x2: String::new(),
                    y2: String::new(),
                    x: String::new(),
                    y: String::new(),
                },
                QUADRATIC_BÉZIER_CURVE_TO => PathOps::QuadraticBézierCurveTo {
                    x1: String::new(),
                    y1: String::new(),
                    x: String::new(),
                    y: String::new(),
                },
                SHORTHAND_QUADRATIC_BÉZIER_CURVE_TO => PathOps::ShorthandQuadraticBézierCurveTo {
                    x: String::new(),
                    y: String::new(),
                },
                ELLIPTICAL_ARC => PathOps::EllipticalArc {
                    rx: String::new(),
                    ry: String::new(),
                    x_axis_rotation: String::new(),
                    large_arc_flag: String::new(),
                    sweep_flag: String::new(),
                    x: String::new(),
                    y: String::new(),
                },
                _ => panic!("Should never happened (a recognised path ops isn't recognised)!"),
            })
            .collect();
        println!("{:?}", matches);
        let mut temp = vec![];
        temp
    }
    // M (x y)+
    // m (x y)+
    // Z
    // z
    // L (x y)+
    // l (x y)+
    // H x+
    // h x+
    // V y+
    // v y+
    // C (x1 y1 x2 y2 x y)+
    // c (x1 y1 x2 y2 x y)+
    // S (x2 y2 x y)+
    // s (x2 y2 x y)+
    // Q (x1 y1 x y)+
    // q (x1 y1 x y)+
    // T (x y)+
    // t (x y)+
    // A (rx ry x-axis-rotation large-arc-flag sweep-flag x y)+
    // a (rx ry x-axis-rotation large-arc-flag sweep-flag x y)+

    //  *[Mm] *, *(?:x *, *y)+ *
    //  *[Zz] *
    //  *[Ll] *, *(?:x *, *y)+ *
    //  *[Hh] *, *x+ *
    //  *[Vv] *, *y+ *
    //  *[Cc] *, *(?:x1 *, *y1 *, *x2 *, *y2 *, *x *, *y)+ *
    //  *[Ss] *, *(?:x2 *, *y2 *, *x *, *y)+ *
    //  *[Qq] *, *(?:x1 *, *y1 *, *x *, *y)+ *
    //  *[Tt] *, *(?:x *, *y)+ *
    //  *[Aa] *, *(?:rx *, *ry *, *x-axis-rotation *, *large-arc-flag *, *sweep-flag *, *x *, *y)+ *
}
use svg_path::*;

const EXAMPLE_OLD: &str = r#"
<!DOCTYPE htmll>
<html len="lol"></html>
<head>
    <meta charset="UTF-8" />
    <title>title</title>
</head>
<body len="lol" lezan="lol"></body>
"#;
const EXAMPLE: &str = r###"
<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="revisit-after" content="7 Days"/>
        <meta name="dcterms.rightsHolder" content="copyright (c) RexEgg.com --- ALL RIGHTS RESERVED"/>
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <link rel="icon" href="https://yu8.us/icons/icon_rexegg.png">
        <link rel="stylesheet" href="https://yu8.us/0_mycss-211224.html?cat=regex&com=1" media="screen,print">
        <link rel="stylesheet" href="https://yu8.us/google_fonts/google-lato-merriweather.css" rel="stylesheet">
        <meta name="description" content="Regular Expressions Syntax Reference. Includes tables showing syntax, examples and matches."/>
        <title>Regex Cheat Sheet</title>
        <!--[if lt IE 9]>
<script src="//html5shiv.googlecode.com/svn/trunk/html5.js"></script>
<![endif]-->
        <script src='https://www.google.com/recaptcha/api.js'></script>
    </head>
    <body>
        <header class="sitebanner" id="the_banner""javascript:void(0)" onclick="close_left_menu()"></header>
        <nav class="topmenu">
            <!-- Top Menu -->
            <table cellpadding="0" class="glossymenu mobile_only">
                <col class="w80"/>
                <col class="w570"/>
                <tr>
                    <td style="vertical-align:top;">
                        <span class="hamburger" onclick="open_left_menu()">
                            <img class="w50" src="data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiA/PjwhRE9DVFlQRSBzdmcgIFBVQkxJQyAnLS8vVzNDLy9EVEQgU1ZHIDEuMS8vRU4nICAnaHR0cDovL3d3dy53My5vcmcvR3JhcGhpY3MvU1ZHLzEuMS9EVEQvc3ZnMTEuZHRkJz48c3ZnIGhlaWdodD0iMzJweCIgaWQ9IkxheWVyXzEiIHN0eWxlPSJlbmFibGUtYmFja2dyb3VuZDpuZXcgMCAwIDMyIDMyOyIgdmVyc2lvbj0iMS4xIiB2aWV3Qm94PSIwIDAgMzIgMzIiIHdpZHRoPSIzMnB4IiB4bWw6c3BhY2U9InByZXNlcnZlIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIj48cGF0aCBkPSJNNCwxMGgyNGMxLjEwNCwwLDItMC44OTYsMi0ycy0wLjg5Ni0yLTItMkg0QzIuODk2LDYsMiw2Ljg5NiwyLDhTMi44OTYsMTAsNCwxMHogTTI4LDE0SDRjLTEuMTA0LDAtMiwwLjg5Ni0yLDIgIHMwLjg5NiwyLDIsMmgyNGMxLjEwNCwwLDItMC44OTYsMi0yUzI5LjEwNCwxNCwyOCwxNHogTTI4LDIySDRjLTEuMTA0LDAtMiwwLjg5Ni0yLDJzMC44OTYsMiwyLDJoMjRjMS4xMDQsMCwyLTAuODk2LDItMiAgUzI5LjEwNCwyMiwyOCwyMnoiIHN0eWxlPSJmaWxsOiNmZmY7Ii8+PC9zdmc+Cg==" alt="Menu">
                        </span>
                    </td>
                    <td style="vertical-align:top;">
                        &#x2B05;Menu: 
                        <i>
                            <b>All</b>
                        </i>
                        the pages<br>quick links &#x2B07;
                    </td>
                </tr>
            </table>
            <ul class="glossymenu">
                <li class="current">
                    <a rel="nofollow" href=".">
                        <b>Fundamentals</b>
                    </a>
                </li>
                <li>
                    <a rel="nofollow" href="regex-disambiguation.html">
                        <b>Black Belt Program</b>
                    </a>
                </li>
                <li>
                    <a rel="nofollow" href="regex-tricks.html">
                        <b>Regex in Action</b>
                    </a>
                </li>
                <li>
                    <a rel="nofollow" href="regex-humor.html">
                        <b>Humor &amp;More</b>
                    </a>
                </li>
                <li>
                    <a rel="nofollow" href="regex-consultant.html">
                        <b>Ask Rex</b>
                    </a>
                </li>
            </ul>
        </nav>
        <!-- Top Menu -->
        <div class="wrap_3cols">
            <div class="wrap_2leftcols">
                <article id="the_article""javascript:void(0)" onclick="close_left_menu()">
                    <br/>
                    <h1>Quick-Start: Regex Cheat Sheet</h1>
                    <div class="cinter">
                        <a rel="nofollow" href="http://www.copyscape.com/" target="_blank">
                            <img src="https://d1go27vtttaqyn.cloudfront.net/copyscape-white.gif" alt="Page copy protected against web site content
        infringement by Copyscape" title="Do not copy content from the page.
        Plagiarism will be detected by Copyscape." width="234" height="16"/>
                        </a>
                    </div>
                    <br/>
                    <script type="text/javascript">
                        function open_left_menu() {
                            document.getElementById("the_leftmenu").style.width = "33%";
                            /* document.getElementById("the_leftmenu").style.margin = "0 67% 0 0"; */
                        }

                        /* Set the width of the side navigation to 0 and the left margin of the page content to 0 */
                        function close_left_menu() {
                            var close_button = document.getElementById("mobile_close_button");
                            if (window.getComputedStyle(close_button).display !== "none") {
                                document.getElementById("the_leftmenu").style.width = "0";
                                document.getElementById("the_article").style.marginLeft = "0";
                                document.getElementById("the_banner").style.marginLeft = "0";
                                document.getElementById("the_top_nav").style.marginLeft = "0";
                            }
                        }
                    </script>
                    The tables below are a reference to basic regex. While reading the rest of the site, when in doubt, you can always come back and look here. (It you want a bookmark, here's a direct link to the <a href="http://www.rexegg.com/regex-quickstart.html#ref" target="_blank">regex reference tables</a>
                    ). I encourage you to print the tables so you have a cheat sheet on your desk for quick reference.<br/>
                    <br/>
                    The tables are not exhaustive, for two reasons. First, every regex flavor is different, and I didn't want to crowd the page with overly exotic syntax. For a full reference to the particular regex flavors you'll be using, it's always best to go straight to the source. In fact, for some regex engines (such as Perl, PCRE, Java and .NET) you may want to check once a year, as their creators often introduce new features. 
<br/>
                    <br/>
                    The other reason the tables are not exhaustive is that I wanted them to serve as a quick introduction to regex. If you are a complete beginner, you should get a firm grasp of basic regex syntax just by reading the examples in the tables. I tried to introduce features in a logical order and to keep out oddities that I've never seen in actual use, such as the "bell character". With these tables as a jumping board, you will be able to advance to mastery by exploring the other pages on the site.
<br/>
                    <br/>
                    <h2>How to use the tables</h2>
                    The tables are meant to serve as an accelerated regex course, and they are meant to be read slowly, one line at a time. On each line, in the leftmost column, you will find a new element of regex syntax. The next column, "Legend", explains what the element means (or encodes) in the regex syntax. The next two columns work hand in hand: the "Example" column gives a valid regular expression that uses the element, and the "Sample Match" column presents a text string that could be matched by the regular expression.<br/>
                    <br/>
                    You can read the tables online, of course, but if you suffer from even the mildest case of online-ADD (attention deficit disorder), like most of us &hellip;Well then, I highly recommend you print them out. You'll be able to study them slowly, and to use them as a cheat sheet later, when you are reading the rest of the site or experimenting with your own regular expressions.
<br/>
                    <br/>
                    Enjoy!
<br/>
                    <br/>
                    If you overdose, make sure not to miss the next page, which comes back down to Earth and talks about some really cool stuff: 
                    <a href="regex-uses.html">
                        <b>The 1001 ways to use Regex</b>
                    </a>
                    .
<br/>
                    <br/>
                    <br/>
                    <a id="ref"></a>
                    <h2>Regex Accelerated Course and Cheat Sheet</h2>
                    For easy navigation, here are some jumping points to various sections of the page:<br/>
                    <br/>
                    ✽ <a href="#chars">Characters</a>
                    <br/>
                    ✽ <a href="#quantifiers">Quantifiers</a>
                    <br/>
                    ✽ <a href="#morechars">More Characters</a>
                    <br/>
                    ✽ <a href="#logic">Logic</a>
                    <br/>
                    ✽ <a href="#whitespace">More White-Space</a>
                    <br/>
                    ✽ <a href="#morequants">More Quantifiers</a>
                    <br/>
                    ✽ <a href="#classes">Character Classes</a>
                    <br/>
                    ✽ <a href="#anchors">Anchors and Boundaries</a>
                    <br/>
                    ✽ <a href="#posix">POSIX Classes</a>
                    <br/>
                    ✽ <a href="#modifiers">Inline Modifiers</a>
                    <br/>
                    ✽ <a href="#lookarounds">Lookarounds</a>
                    <br/>
                    ✽ <a href="#classoperations">Character Class Operations</a>
                    <br/>
                    ✽ <a href="#other">Other Syntax</a>
                    <br/>
                    <br/>
                    <br/>
                    <a id="chars"></a>
                    <span class="em8">
                        <a href="#chars">(direct link)</a>
                    </span>
                    <br/>
                    <h2>Characters</h2>
                    <table " border=" 0 " style=" table-layout:fixed;"><tr><th class=" w100 " scope=" col ">Character</th><th class=" w200 " scope=" col ">Legend</th><th class=" w150 " scope=" col ">Example</th><th class=" w150 " scope=" col ">Sample Match</th></tr><tr class=" brown "><td><span class=" mono ">\d</span></td><td>Most engines: one digit<br />from 0 to 9</td><td>file_\d\d</td><td>file_25</td></tr><tr class=" beige "><td><span class=" mono ">\d</span></td><td>.NET, Python 3: one Unicode digit in any script</td><td>file_\d\d</td><td>file_9੩</td></tr><tr class=" brown "><td><span class=" mono ">\w</span></td><td>Most engines: " word character ": ASCII letter, digit or underscore</td><td>\w-\w\w\w</td><td>A-b_1</td></tr><tr class=" beige "><td><span class=" mono ">\w</span></td><td>.Python 3: " word character ": Unicode letter, ideogram, digit, or underscore</td><td>\w-\w\w\w</td><td>字-ま_۳</td></tr><tr class=" brown "><td><span class=" mono ">\w</span></td><td>.NET: " word character ": Unicode letter, ideogram, digit, or connector</td><td>\w-\w\w\w</td><td>字-ま‿۳</td></tr><tr class=" beige "><td><span class=" mono ">\s</span></td><td>Most engines: " whitespace character ": space, tab, newline, carriage return, vertical tab</td><td>a\sb\sc</td><td>a b<br />c</td></tr><tr class=" brown "><td><span class=" mono ">\s</span></td><td>.NET, Python 3, JavaScript: " whitespace character ": any Unicode separator</td><td>a\sb\sc</td><td>a b<br />c</td></tr><tr class=" beige "><td><span class=" mono ">\D</span></td><td>One character that is not a <i>digit</i> as defined by your engine's <i>\d</i></td><td>\D\D\D</td><td>ABC</td></tr><tr class=" brown "><td><span class=" mono ">\W</span></td><td>One character that is not a <i>word character</i> as defined by your engine's <i>\w</i></td><td>\W\W\W\W\W</td><td>*-+=)</td></tr><tr class=" beige "><td><span class=" mono ">\S</span></td><td>One character that is not a <i>whitespace character</i> as defined by your engine's <i>\s</i></td><td>\S\S\S\S</td><td>Yoyo</td></tr></table><br /><br />



<a id=" quantifiers "></a>
<span class=" em8 "><a href=" #quantifiers ">(direct link)</a></span><br />
<h2>Quantifiers</h2>


<table " border="0" style="table-layout:fixed;">
                        <tr>
                            <th class="w100" scope="col">Quantifier</th>
                            <th class="w200" scope="col">Legend</th>
                            <th class="w150" scope="col">Example</th>
                            <th class="w150" scope="col">Sample Match</th>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">+</span>
                            </td>
                            <td>One or more</td>
                            <td>Version \w-\w+</td>
                            <td>Version A-b1_1</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">{3}</span>
                            </td>
                            <td>Exactly three times</td>
                            <td>\D{3}</td>
                            <td>ABC</td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">{2,4}</span>
                            </td>
                            <td>Two to four times</td>
                            <td>\d{2,4}</td>
                            <td>156</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">{3,}</span>
                            </td>
                            <td>Three or more times</td>
                            <td>\w{3,}</td>
                            <td>regex_tutorial</td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">*</span>
                            </td>
                            <td>Zero or more times</td>
                            <td>A*B*C*</td>
                            <td>AAACC</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">?</span>
                            </td>
                            <td>Once or none</td>
                            <td>plurals?</td>
                            <td>plural</td>
                        </tr>
                    </table>
                    <br/>
                    <br/>
                    <a id="morechars"></a>
                    <span class="em8">
                        <a href="#morechars">(direct link)</a>
                    </span>
                    <br/>
                    <h2>More Characters</h2>
                    <table " border=" 0 " style=" table-layout:fixed;"><tr><th class=" w100 " scope=" col ">Character</th><th class=" w200 " scope=" col ">Legend</th><th class=" w150 " scope=" col ">Example</th><th class=" w150 " scope=" col ">Sample Match</th></tr><tr class=" brown "><td><span class=" mono "><b>.</b></span></td><td>Any character except line break</td><td>a.c</td><td>abc</td></tr><tr class=" beige "><td><span class=" mono "><b>.</b></span></td><td>Any character except line break</td><td>.*</td><td>whatever, man.</td></tr><tr class=" brown "><td><span class=" mono ">\<b>.</b></span></td><td>A period (special character: needs to be escaped by a \)</td><td>a\.c</td><td>a.c</td></tr><tr class=" beige "><td><span class=" mono ">\</span></td><td>Escapes a special character</td><td>\.\*\+\?&nbsp;&nbsp;&nbsp;&nbsp;\$\^\/\&#92;</td><td>.*+?&nbsp;&nbsp;&nbsp;&nbsp;$^/&#92;</td></tr><tr class=" brown "><td><span class=" mono ">\</span></td><td>Escapes a special character</td><td>\[\{\(\)\}\]</td><td>[{()}]</td></tr></table><br /><br />



<a id=" logic "></a>
<span class=" em8 "><a href=" #logic ">(direct link)</a></span><br />
<h2>Logic</h2>


<table " border="0" style="table-layout:fixed;">
                        <tr>
                            <th class="w100" scope="col">Logic</th>
                            <th class="w200" scope="col">Legend</th>
                            <th class="w150" scope="col">Example</th>
                            <th class="w150" scope="col">Sample Match</th>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">|</span>
                            </td>
                            <td>Alternation / OR operand</td>
                            <td>22|33</td>
                            <td>33</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">( … )</span>
                            </td>
                            <td>Capturing group</td>
                            <td>A(nt|pple)</td>
                            <td>Apple (captures "pple")</td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">\1</span>
                            </td>
                            <td>Contents of Group 1</td>
                            <td>r(\w)g\1x</td>
                            <td>regex</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">\2</span>
                            </td>
                            <td>Contents of Group 2</td>
                            <td>(\d\d)\+(\d\d)=\2\+\1</td>
                            <td>12+65=65+12</td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">(?: … )</span>
                            </td>
                            <td>Non-capturing group</td>
                            <td>A(?:nt|pple)</td>
                            <td>Apple</td>
                        </tr>
                    </table>
                    <br/>
                    <br/>
                    <a id="whitespace"></a>
                    <span class="em8">
                        <a href="#whitespace">(direct link)</a>
                    </span>
                    <br/>
                    <h2>More White-Space</h2>
                    <table " border=" 0 " style=" table-layout:fixed;"><tr><th class=" w100 " scope=" col ">Character</th><th class=" w200 " scope=" col ">Legend</th><th class=" w150 " scope=" col ">Example</th><th class=" w150 " scope=" col ">Sample Match</th></tr><tr class=" brown "><td><span class=" mono ">\t</span></td><td>Tab</td><td>T\t\w{2}</td><td>T&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;ab</td></tr><tr class=" beige "><td><span class=" mono ">\r</span></td><td>Carriage return character</td><td>see below</td><td></td></tr><tr class=" brown "><td><span class=" mono ">\n</span></td><td>Line feed character</td><td>see below</td><td></td></tr><tr class=" beige "><td><span class=" mono ">\r\n</span></td><td>Line separator on Windows</td><td>AB\r\nCD</td><td>AB<br />CD</td></tr><tr class=" brown "><td><span class=" mono ">\N</span></td><td>Perl, PCRE (C, PHP, R…): one character that is not a line break</td><td>\N+</td><td>ABC</td></tr><tr class=" beige "><td><span class=" mono ">\h</span></td><td>Perl, PCRE (C, PHP, R…), Java: one horizontal whitespace character: tab or Unicode space separator</td><td></td><td></td></tr><tr class=" brown "><td><span class=" mono ">\H</span></td><td>One character that is not a horizontal whitespace</td><td></td><td></td></tr><tr class=" beige "><td><span class=" mono ">\v</span></td><td>.NET, JavaScript, Python, Ruby: vertical tab</td><td></td><td></td></tr><tr class=" brown "><td><span class=" mono ">\v</span></td><td>Perl, PCRE (C, PHP, R…), Java: one vertical whitespace character: line feed, carriage return, vertical tab, form feed, paragraph or line separator</td><td></td><td></td></tr><tr class=" beige "><td><span class=" mono ">\V</span></td><td>Perl, PCRE (C, PHP, R…), Java: any character that is not a vertical whitespace</td><td></td><td></td></tr><tr class=" brown "><td><span class=" mono ">\R</span></td><td>Perl, PCRE (C, PHP, R…), Java: one line break (carriage return + line feed pair, and all the characters matched by \v)</td><td></td><td></td></tr></table><br /><br />



<a id=" morequants "></a>
<span class=" em8 "><a href=" #morequants ">(direct link)</a></span><br />
<h2>More Quantifiers</h2>


<table " border="0" style="table-layout:fixed;">
                        <tr>
                            <th class="w100" scope="col">Quantifier</th>
                            <th class="w200" scope="col">Legend</th>
                            <th class="w150" scope="col">Example</th>
                            <th class="w150" scope="col">Sample Match</th>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">+</span>
                            </td>
                            <td>The + (one or more) is "greedy"</td>
                            <td>\d+</td>
                            <td>12345</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">?</span>
                            </td>
                            <td>Makes quantifiers "lazy"</td>
                            <td>\d+?</td>
                            <td>
                                1 in <b>1</b>
                                2345
                            </td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">*</span>
                            </td>
                            <td>The * (zero or more) is "greedy"</td>
                            <td>A*</td>
                            <td>AAA</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">?</span>
                            </td>
                            <td>Makes quantifiers "lazy"</td>
                            <td>A*?</td>
                            <td>empty in AAA</td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">{2,4}</span>
                            </td>
                            <td>Two to four times, "greedy"</td>
                            <td>\w{2,4}</td>
                            <td>abcd</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">?</span>
                            </td>
                            <td>Makes quantifiers "lazy"</td>
                            <td>\w{2,4}?</td>
                            <td>
                                ab in <b>ab</b>
                                cd
                            </td>
                        </tr>
                    </table>
                    <br/>
                    <br/>
                    <a id="classes"></a>
                    <span class="em8">
                        <a href="#classes">(direct link)</a>
                    </span>
                    <br/>
                    <h2>Character Classes</h2>
                    <table " border=" 0 " style=" table-layout:fixed;"><tr><th class=" w100 " scope=" col ">Character</th><th class=" w200 " scope=" col ">Legend</th><th class=" w150 " scope=" col ">Example</th><th class=" w150 " scope=" col ">Sample Match</th></tr><tr class=" brown "><td><span class=" mono ">[ … ]</span></td><td>One of the characters in the brackets</td><td>[AEIOU]</td><td>One uppercase vowel</td></tr><tr class=" beige "><td><span class=" mono ">[ … ]</span></td><td>One of the characters in the brackets</td><td>T[ao]p</td><td><i>Tap</i> or <i>Top</i></td></tr><tr class=" brown "><td><span class=" mono ">-</span></td><td>Range indicator</td><td>[a-z]</td><td>One lowercase letter</td></tr><tr class=" beige "><td><span class=" mono ">[x-y]</span></td><td>One of the characters in the range from x to y</td><td>[A-Z]+</td><td>GREAT</td></tr><tr class=" brown "><td><span class=" mono ">[ … ]</span></td><td>One of the characters in the brackets</td><td>[AB1-5w-z]</td><td>One of either: A,B,1,2,3,4,5,w,x,y,z</td></tr><tr class=" beige "><td><span class=" mono ">[x-y]</span></td><td>One of the characters in the range from x to y</td><td>[&ensp;-~]+</td><td>Characters in the printable section of the <a href=" http://www.asciitable.com /" target=" _blank ">ASCII table</a>.</td></tr><tr class=" brown "><td><span class=" mono ">[^x]</span></td><td>One character that is not x</td><td>[^a-z]{3}</td><td>A1!</td></tr><tr class=" beige "><td><span class=" mono ">[^x-y]</span></td><td>One of the characters <b>not</b> in the range from x to y</td><td>[^&ensp;-~]+</td><td>Characters that are <b>not</b> in the printable section of the <a href=" http://www.asciitable.com /" target=" _blank ">ASCII table</a>.</td></tr><tr class=" brown "><td><span class=" mono ">[\d\D]</span></td><td>One character that is a digit or a non-digit</td><td>[\d\D]+</td><td>Any characters, inc-<br />luding new lines, which the regular dot doesn't match</td></tr><tr class=" beige "><td><span class=" mono ">[\x41]</span></td><td>Matches the character at hexadecimal position 41 in the ASCII table, i.e. A</td><td>[\x41-\x45]{3}</td><td>ABE</td></tr></table><br /><br />



<a id=" anchors "></a>
<span class=" em8 "><a href=" #anchors ">(direct link)</a></span><br />
<h2><a href=" regex-anchors.html ">Anchors</a> and <a href=" regex-boundaries.html ">Boundaries</a></h2>


<table " border="0" style="table-layout:fixed;">
                        <tr>
                            <th class="w100" scope="col">Anchor</th>
                            <th class="w200" scope="col">Legend</th>
                            <th class="w150" scope="col">Example</th>
                            <th class="w150" scope="col">Sample Match</th>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">^</span>
                            </td>
                            <td>
                                <a href="regex-anchors.html#caret">Start of string</a>
                                or <a href="regex-anchors.html#carmulti">start of line</a>
                                depending on multiline mode. (But when [^inside brackets], it means "not")
                            </td>
                            <td>^abc .*</td>
                            <td>abc (line start)</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">$</span>
                            </td>
                            <td>
                                <a href="regex-anchors.html#dollar">End of string</a>
                                or <a href="regex-anchors.html#eol">end of line</a>
                                depending on multiline mode. Many engine-dependent subtleties.
                            </td>
                            <td>.*? the end$</td>
                            <td>this is the end</td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">\A</span>
                            </td>
                            <td>
                                <a href="regex-anchors.html#A">Beginning of string</a>
                                <br/>(all major engines except JS)
                            </td>
                            <td>\Aabc[\d\D]*</td>
                            <td>
                                abc (string...<br/>...start)
                            </td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">\z</span>
                            </td>
                            <td>
                                <a href="regex-anchors.html#z">Very end of the string</a>
                                <br/>Not available in Python and JS
                            </td>
                            <td>the end\z</td>
                            <td>
                                this is...\n...<b>the end</b>
                            </td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">\Z</span>
                            </td>
                            <td>
                                <a href="regex-anchors.html#Z">End of string</a>
                                or (except Python) before final line break<br/>Not available in JS
                            </td>
                            <td>the end\Z</td>
                            <td>
                                this is...\n...<b>the end</b>
                                \n
                            </td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">\G</span>
                            </td>
                            <td>
                                <a href="regex-anchors.html#G">Beginning of String or End of Previous Match</a>
                                <br/>.NET, Java, PCRE (C, PHP, R…), Perl, Ruby
                            </td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">\b</span>
                            </td>
                            <td>
                                <a href="regex-boundaries.html#wordboundary">Word boundary</a>
                                <br/>Most engines: position where one side only is an ASCII letter, digit or underscore
                            </td>
                            <td>Bob.*\bcat\b</td>
                            <td>Bob ate the cat</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">\b</span>
                            </td>
                            <td>
                                <a href="regex-boundaries.html#wordboundary">Word boundary</a>
                                <br/>.NET, Java, Python 3, Ruby: position where one side only is a Unicode letter, digit or underscore
                            </td>
                            <td>Bob.*\b\кошка\b</td>
                            <td>Bob ate the кошка</td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">\B</span>
                            </td>
                            <td>
                                <a href="regex-boundaries.html#notb">Not a word boundary</a>
                            </td>
                            <td>c.*\Bcat\B.*</td>
                            <td>copycats</td>
                        </tr>
                    </table>
                    <br/>
                    <br/>
                    <a id="posix"></a>
                    <span class="em8">
                        <a href="#posix">(direct link)</a>
                    </span>
                    <br/>
                    <h2>POSIX Classes</h2>
                    <table " border=" 0 " style=" table-layout:fixed;"><tr><th class=" w100 " scope=" col ">Character</th><th class=" w200 " scope=" col ">Legend</th><th class=" w150 " scope=" col ">Example</th><th class=" w150 " scope=" col ">Sample Match</th></tr><tr class=" brown "><td><span class=" mono ">[:alpha:]</span></td><td>PCRE (C, PHP, R…): ASCII letters A-Z and a-z</td><td>[8[:alpha:]]+</td><td>WellDone88</td></tr><tr class=" beige "><td><span class=" mono ">[:alpha:]</span></td><td>Ruby 2: Unicode letter or ideogram</td><td>[[:alpha:]\d]+</td><td>кошка99</td></tr><tr class=" brown "><td><span class=" mono ">[:alnum:]</span></td><td>PCRE (C, PHP, R…): ASCII digits and letters A-Z and a-z</td><td>[[:alnum:]]{10}</td><td>ABCDE12345</td></tr><tr class=" beige "><td><span class=" mono ">[:alnum:]</span></td><td>Ruby 2: Unicode digit, letter or ideogram</td><td>[[:alnum:]]{10}</td><td>кошка90210</td></tr><tr class=" brown "><td><span class=" mono ">[:punct:]</span></td><td>PCRE (C, PHP, R…): ASCII punctuation mark</td><td>[[:punct:]]+</td><td>?!.,:;</td></tr><tr class=" beige "><td><span class=" mono ">[:punct:]</span></td><td>Ruby: Unicode punctuation mark</td><td>[[:punct:]]+</td><td>‽,:〽⁆</td></tr></table><br /><br />



<a id=" modifiers "></a>
<span class=" em8 "><a href=" #modifiers ">(direct link)</a></span><br />
<h2><a href=" regex-modifiers.html ">Inline Modifiers</a></h2>


None of these are supported in JavaScript. In Ruby, beware of <span class=" socode ">(?s)</span> and <span class=" socode ">(?m)</span>.
<br />
<table " border="0" style="table-layout:fixed;">
                        <tr>
                            <th class="w100" scope="col">Modifier</th>
                            <th class="w200" scope="col">Legend</th>
                            <th class="w150" scope="col">Example</th>
                            <th class="w150" scope="col">Sample Match</th>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">(?i)</span>
                            </td>
                            <td>
                                <a href="regex-modifiers.html#i">Case-insensitive mode</a>
                                <br/>(except JavaScript)
                            </td>
                            <td>(?i)Monday</td>
                            <td>monDAY</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">(?s)</span>
                            </td>
                            <td>
                                <a href="regex-modifiers.html#dotall">DOTALL mode</a>
                                (except JS and Ruby). The dot (.) matches new line characters (\r\n). Also known as "single-line mode" because the dot treats the entire input as a single line
                            </td>
                            <td>(?s)From A.*to Z</td>
                            <td>
                                From A<br/>to Z
                            </td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">(?m)</span>
                            </td>
                            <td>
                                <a href="regex-modifiers.html#multiline">Multiline mode</a>
                                <br/>(except Ruby and JS) ^ and $ match at the beginning and end of every line
                            </td>
                            <td>(?m)1\r\n^2$\r\n^3$</td>
                            <td>
                                1<br/>
                                2<br/>3
                            </td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">(?m)</span>
                            </td>
                            <td>
                                <a href="regex-modifiers.html#rubym">In Ruby</a>
                                : the same as (?s) in other engines, i.e. DOTALL mode, i.e. dot matches line breaks
                            </td>
                            <td>(?m)From A.*to Z</td>
                            <td>
                                From A<br/>to Z
                            </td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">(?x)</span>
                            </td>
                            <td>
                                <a href="regex-modifiers.html#freespacing">Free-Spacing Mode mode</a>
                                <br/>(except JavaScript). Also known as comment mode or whitespace mode
                            </td>
                            <td>
                                (?x) # this is a<br/>
                                # comment<br/>
                                abc # write on multiple<br/>
                                # lines<br/>
                                [ ]d # spaces must be<br/># in brackets
                            </td>
                            <td>abc d</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">(?n)</span>
                            </td>
                            <td>
                                <a href="regex-modifiers.html#n">.NET, PCRE 10.30+: named capture only</a>
                            </td>
                            <td>
                                Turns all (parentheses) into non-capture groups. To capture, use <a href="regex-capture.html#namedgroups">named groups</a>
                                .
                            </td>
                            <td></td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">(?d)</span>
                            </td>
                            <td>
                                <a href="regex-modifiers.html#d">Java: Unix linebreaks only</a>
                            </td>
                            <td>The dot and the ^ and $ anchors are only affected by \n</td>
                            <td></td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">(?^)</span>
                            </td>
                            <td>
                                <a href="regex-disambiguation.html#unset-all">PCRE 10.32+: unset modifiers</a>
                            </td>
                            <td>
                                Unsets <span class="mono">ismnx</span>
                                modifiers
                            </td>
                            <td></td>
                        </tr>
                    </table>
                    <br/>
                    <br/>
                    <a id="lookarounds"></a>
                    <span class="em8">
                        <a href="#lookarounds">(direct link)</a>
                    </span>
                    <br/>
                    <h2>
                        <a href="regex-lookarounds.html">Lookarounds</a>
                    </h2>
                    <table " border=" 0 " style=" table-layout:fixed;"><tr><th class=" w100 " scope=" col ">Lookaround</th><th class=" w200 " scope=" col ">Legend</th><th class=" w150 " scope=" col ">Example</th><th class=" w150 " scope=" col ">Sample Match</th></tr><tr class=" brown "><td><span class=" mono ">(?=…)</span></td><td><a href=" regex-disambiguation.html#lookahead ">Positive lookahead</a></td><td>(?=\d{10})\d{5}</td><td>01234 in <b>01234</b>56789</td></tr><tr class=" beige "><td><span class=" mono ">(?&lt;=…)</span></td><td><a href=" regex-disambiguation.html#lookbehind ">Positive lookbehind</a></td><td>(?&lt;=\d)cat</td><td>cat in 1<b>cat</b></td></tr><tr class=" brown "><td><span class=" mono ">(?!…)</span></td><td><a href=" regex-disambiguation.html#negative-lookahead ">Negative lookahead</a></td><td>(?!theatre)the\w+</td><td>theme</td></tr><tr class=" beige "><td><span class=" mono ">(?&lt;!…)</span></td><td><a href=" regex-disambiguation.html#negative-lookbehind ">Negative lookbehind</a></td><td>\w{3}(?&lt;!mon)ster</td><td>Munster</td></tr></table><br /><br />



<a id=" classoperations "></a>
<span class=" em8 "><a href=" #classoperations ">(direct link)</a></span><br />
<h2><a href=" regex-class-operations.html ">Character Class Operations</a></h2>


<table " border="0" style="table-layout:fixed;">
                        <tr>
                            <th class="w100" scope="col">Class Operation</th>
                            <th class="w200" scope="col">Legend</th>
                            <th class="w150" scope="col">Example</th>
                            <th class="w150" scope="col">Sample Match</th>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">[…-[…]]</span>
                            </td>
                            <td>.NET: character class subtraction. One character that is in those on the left, but not in the subtracted class.</td>
                            <td>[a-z-[aeiou]]</td>
                            <td>Any lowercase consonant</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">[…-[…]]</span>
                            </td>
                            <td>.NET: character class subtraction.</td>
                            <td>[\p{IsArabic}-[\D]]</td>
                            <td>An Arabic character that is not a non-digit, i.e., an Arabic digit</td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">[…&&[…]]</span>
                            </td>
                            <td>Java, Ruby 2+: character class intersection.  One character that is both in those on the left and in the &&class.</td>
                            <td>[\S&&[\D]]</td>
                            <td>An non-whitespace character that is a non-digit.</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">[…&&[…]]</span>
                            </td>
                            <td>Java, Ruby 2+: character class intersection.</td>
                            <td>[\S&&[\D]&&[^a-zA-Z]]</td>
                            <td>An non-whitespace character that a non-digit and not a letter.</td>
                        </tr>
                        <tr class="wasabi">
                            <td>
                                <span class="mono">[…&&[^…]]</span>
                            </td>
                            <td>Java, Ruby 2+: character class subtraction is obtained by intersecting a class with a negated class</td>
                            <td>[a-z&&[^aeiou]]</td>
                            <td>An English lowercase letter that is not a vowel.</td>
                        </tr>
                        <tr class="greentea">
                            <td>
                                <span class="mono">[…&&[^…]]</span>
                            </td>
                            <td>Java, Ruby 2+: character class subtraction</td>
                            <td>[\p{InArabic}&&[^\p{L}\p{N}]]</td>
                            <td>An Arabic character that is not a letter or a number</td>
                        </tr>
                    </table>
                    <br/>
                    <br/>
                    <a id="other"></a>
                    <span class="em8">
                        <a href="#other">(direct link)</a>
                    </span>
                    <br/>
                    <h2>Other Syntax</h2>
                    <table " border="0 " style="table-layout:fixed;"><tr><th class="w100 " scope="col ">Syntax</th><th class="w200 " scope="col ">Legend</th><th class="w150 " scope="col ">Example</th><th class="w150 " scope="col ">Sample Match</th></tr><tr class="brown "><td><name="K "></name>
<span class="mono ">\K</span></td><td><a href="regex-best-trick.html#bsk " >Keep Out</a><br>
Perl, PCRE (C, PHP, R…), Python's alternate <a href="https://pypi.python.org/pypi/regex " target="_blank "><i>regex</i></a> engine, Ruby 2+: drop everything that was matched so far from the overall match to be returned</td><td>prefix\K\d+</td><td>12</td></tr><tr class="beige "><td><name="blockescape "></name><span class="mono ">\Q…\E</span></td><td>Perl, PCRE (C, PHP, R…), Java: treat anything between the delimiters as a literal string. Useful to escape metacharacters.</td><td>\Q(C++ ?)\E</td><td>(C++ ?)</td></tr></table><br /><br />



<br /><br /><div class="cinter ">
Don't Miss The <a href="regex-style.html " ><span class="redtext em2 "><b>Regex Style Guide</b></span></a><br /><br />
and <a href="regex-best-trick.html " ><span class="redtext em2 "><b>The Best Regex Trick Ever!!!</b></span></a><br />
</div><a href="regex-uses.html " >
<img src="https://d1go27vtttaqyn.cloudfront.net/next_regex.png " class="left " width="125 " height="40 " alt="next " /><br />
<b>&nbsp;The 1001 ways to use Regex</b>
</a>
<br /><br /><br />



<div class="cinter ">
		<a href="regex-consultant.html ">
		<img src="https://d1go27vtttaqyn.cloudfront.net/rightgraphic_rexegg3.png "
        class="tada " width="100 " height="100 " alt="Regex Rex " /><br />
		<b>Ask Rex</b></a>
		<br /><br />
		</div><a id="comlist "></a>
<div class="comment-wrapper "><br /><div class="cinter "><a href="#comform ">Leave a Comment</a></div><div class="cinter redtext mt1 ">1-10 of 19 Threads</div>
<a id="cid3276 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">Appu</span><span class="comment-location "> &ndash; Japan</span></div>
<div class="comment-date ">March 07, 2022 - 19:05</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>You are God of regex !! Thank you so much :)</div><br />
<div class = "comment-text ">This site is absolute gold mine. I once stumbled upon and missed it, now found again&hellip; So happy :D Thank you so much for all your efforts!!</div>
</div>
<a id="cid3106 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">maureen</span><span class="comment-location "> &ndash; san francisco</span></div>
<div class="comment-date ">June 18, 2021 - 16:25</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>absolutely the BEST website for regex</div><br />
<div class = "comment-text ">This is the go-to website for everything on regex. Thank you!</div>
</div>
<a id="cid2825 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">Pythia</span><span class="comment-location "> &ndash; New Zealand</span></div>
<div class="comment-date ">July 15, 2020 - 03:54</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>Very thoughtful and useful cheat sheet</div><br />
<div class = "comment-text ">Unlike lots of other cheat sheets or regex web sites, I was able (without much persistent regex knowledge) to apply the rules and to solve my problem. THANK YOU :)</div>
</div>
<a id="cid2815 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">Mark</span></div>
<div class="comment-date ">July 04, 2020 - 10:14</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>Thanks a lot</div><br />
<div class = "comment-text ">Thanks a lot for the quick guide. It&#039;s really helpful.</div>
</div>
<a id="cid2805 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">Purusharth Amrut</span></div>
<div class="comment-date ">June 10, 2020 - 14:41</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>Very useful site</div><br />
<div class = "comment-text ">Thank you soooooo much for this site. I&#039;m using python regex for natural language processing in sentiment analysis and this helped me a lot.</div>
</div>
<a id="cid2762 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">Alessandro Maiorana</span><span class="comment-location "> &ndash; Italy, Milan</span></div>
<div class="comment-date ">April 15, 2020 - 12:43</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>Thank you! Excellent resource for any student</div><br />
<div class = "comment-text ">Thank you so much for this incredible cheatsheet! It is facilitating a lot my regex learning! God bless you and your passion!</div>
</div>
<a id="cid2753 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">michael</span><span class="comment-location "> &ndash; Bulgaria</span></div>
<div class="comment-date ">April 10, 2020 - 12:43</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>Thank you for doing such a geat work.</div><br />
<div class = "comment-text ">I am now learning regex and for finding such a well organized site is a blessing! You are a good soul! Thank you for everything and stay inspired!</div>
</div>
<a id="cid2702 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">Yuri</span><span class="comment-location "> &ndash; California</span></div>
<div class="comment-date ">November 13, 2019 - 17:39</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>Simple = perfect</div><br />
<div class = "comment-text ">Thanks a lot, saved me tons of time!!!!</div>
</div>
<a id="cid2678 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">Tom</span><span class="comment-location "> &ndash; Europe, Poland</span></div>
<div class="comment-date ">September 30, 2019 - 18:43</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>Congratulations</div><br />
<div class = "comment-text ">Well done, very useful page. Thank you for your effort. T</div>
</div>
<a id="cid2611 "></a>
<div class = "comment ">
<div class="comment-header "><div class="comment-author_location "><span class="comment-author ">Najam</span></div>
<div class="comment-date ">March 25, 2019 - 03:44</div></div>
<div class = "comment-subject "><span class="comment-subject-title ">Subject: </span>Thank you very much</div><br />
<div class = "comment-text ">Hi Rex,<br />
<br />
Thankyou very much for compiling these. I am new to text analytics and is struggling a lot with regex. This is helping me a lot pick up. Great work</div>
</div>
<br /><div class="cinter "><a  rel="nofollow " 
		href="?cp=2#comlist "><img src="https://d1go27vtttaqyn.cloudfront.net/nav_next16.png " class="" width="16 " height="16 " alt="Next " /></a>&nbsp;&nbsp;</div>
</div><br /><br />
<a id="comform "></a><div class="comment-form-wrapper "><div class="cinter redtext mt1 ">Leave a Comment</div><div id="comment-form-div ">
<form action="regex-quickstart.html " method="post " >
<input type="hidden " name="token " value="be68b1498b795e45368f175bbcd63a0a " />
<input type="hidden " name="admin " value="0 " />

<input class="comment-field " type="text "
			name="poster " value="" />
		<label class="comment-required-field ">*</label>
		<label class="comment-label ">Your name</label><br />
<input class="comment-field " type="text "
			name="email " value="" />
		<label class="comment-required-field ">*</label>
		<label class="comment-label ">Email (it will not be shown)</label><br />
<input class="comment-field " type="text "
			name="location " value="" />
		<label class="comment-label ">Your location</label><br /><br />
<label class="comment-label ">Subject: </label>
			<input class="comment-subject-field " type="text "
			name="subject " value=""  maxlength="128 " /><br /><br />
<div class="cinter blue ">
		<b>All comments are moderated.<br />
		Link spammers, this won't work for you.</b></div>
<textarea name="comment "></textarea><br />To prevent automatic spam, may I gently ask that you go through these crazy hoops…<br />
        <div class="captcha_wrapper ">
		<div class="g-recaptcha " data-sitekey="6Lc2WE8UAAAAABFS5ks4OB6onjL6EJOP_kDE7zTZ "></div>
	    </div>
<div id="comment-form-submit-line ">
<input class="comment-submit " type="submit "
			name="submitcomment " value="Submit "
			onmouseout="this.className='comment-submit'"
			onmouseover="this.className='comment-submit-hover'" /></div></form>

</div></div><br />
	
</article>
<nav id='the_leftmenu' class='leftmenu fadein'><a id="mobile_close_button " href="javascript:void(0)" class="mobile_only mob_leftmenu_close_button " onclick="close_left_menu()">&times;</a>

<b>Fundamentals</b><br /><ul><li><a  href=".">Regex Tutorial</a></li><li><a  href="regex-vs-regular-expression.html ">Regex vs. Regex</a></li><li><a class="thispage " href="regex-quickstart.html ">Quick Reference</a></li><li><a  href="regex-uses.html ">100 Uses for Regex</a></li><li><a  href="regex-style.html ">Regex Style Guide</a></li></ul><br />
<b>Black Belt Program</b> <br /><ul><li><a  href="regex-disambiguation.html ">All <span class="mono ">(? &hellip; ) Syntax</span></a></li><li><a  href="regex-boundaries.html ">Boundaries++</a></li><li><a  href="regex-anchors.html ">Anchors</a></li><li><a  href="regex-capture.html ">Capture &amp; Back</a></li><li><a  href="regex-modifiers.html ">Flags &amp; Modifiers</a></li><li><a  href="regex-lookarounds.html ">Lookarounds</a></li><li><a  href="regex-quantifiers.html ">Quantifiers</a></li><li><a  href="regex-explosive-quantifiers.html ">Explosive Quantifiers</a></li><li><a  href="regex-conditionals.html ">Conditionals</a></li><li><a  href="regex-recursion.html ">Recursion</a></li><li><a  href="regex-class-operations.html ">Class Operations</a></li><li><a  href="backtracking-control-verbs.html ">Backtracking Control</a></li><li><a  href="regex-gotchas.html ">Regex <i>Gotchas</i></a></li><li><a  href="regex-tricks.html ">Syntax Tricks</a></li><li><a  href="pcre-callouts.html ">PCRE Callouts</a></li><li><a  href="regex-quantifier-capture.html ">Quantifier capture</a></li></ul><br />
<b>Regex in Action</b> <br /><div class="">For awesome tricks:<br />scroll down!</div><ul><li><a  href="regex-cookbook.html ">Cookbook</a></li><li><a  href="regex-interesting-character-classes.html ">Cool Regex Classes</a></li><li><a  href="regex-optimizations.html ">Regex Optimizations</a></li><li><a  href="pcregrep-pcretest.html ">PCRE: Grep and Test</a></li><li><a  href="regex-perl-one-liners.html ">Perl One-Liners</a></li><li><a  href="regex-firefox-shortcuts.html ">Amazing Shortcuts</a></li></ul><br />
<b>Tools &amp; More</b> <br /><ul><li><a  href="regex-tools.html ">Regex Tools</a></li><li><a  href="regexbuddy-tutorial.html ">RegexBuddy</a></li><li><a  href="regex-humor.html ">Regex Humor</a></li><li><a  href="regex-books.html ">Regex Books &amp; Links</a></li></ul><br />
<b>Tricks</b> <br /><ul><li><a  href="regex-best-trick.html ">The Best Regex Trick</a></li><li><a  href="regex-trick-conditional-replacement.html ">Conditional Sub</a></li><li><a  href="regex-trick-line-numbers.html ">Line Numbers</a></li><li><a  href="regex-trick-numbers-in-english.html ">Numbers in English</a></li></ul><br />
<b>Languages</b> <br /><ul><li><a  href="pcre-documentation.html ">PCRE Doc &amp; Log</a></li><li><a  href="regex-perl.html ">Regex with Perl</a></li><li><a  href="regex-csharp.html ">Regex with C#</a></li><li><a  href="regex-php.html ">Regex with PHP</a></li><li><a  href="regex-python.html ">Regex with Python</a></li><li><a  href="regex-java.html ">Regex with Java</a></li><li><a  href="regex-javascript.html ">Regex with JavaScript</a></li><li><a  href="regex-ruby.html ">Regex with Ruby</a></li><li><a  href="regex-vbnet.html ">Regex with VB.NET</a></li></ul>
<br />




</nav>
</div><!-- wrap_2leftcols -->
</div><!-- wrap_3cols -->
	<div class="topanchor desktop_only ">
	<aside class="rightcolumn fadein "><table class="cintable ">
            <tr><td style="text-align:center;">
            <a rel="nofollow " href="http://www.amazon.com/dp/0596528124?tag=onamazon-20 "
            target="_blank ">
            <img src="https://d1go27vtttaqyn.cloudfront.net/c_mastering-regex.jpg "
            class="pulse1st " 
			height="160 " width="116 " alt="Matering Regular Expressions " />
            <br /><span class="greytext em9 "><b>A must-read</b></span><br /><br /></a></td></tr></table><div class="cinter greytext ">
					<b>
					RegexBuddy 4 is<br />
					the <i>best</i> regex tool!<br />
					</b>
					<div class="em9 pulse2nd strong ">
					<a href="regexbuddy-trial.html "	><span class="redtext under ">Get the Free Trial</span></a><br />
					<a href="regexbuddy-tutorial.html "	><span class="redtext under ">Huge RB Tutorial</span></a>
					</div>
					<br />
					<div class="dtada " style="margin:0 0 -0.5em 0;">
					<img src="https://d1go27vtttaqyn.cloudfront.net/rightgraphic_rexegg3.png "
                    class="tada " width="100 " height="100 " alt="Regex Rex " />
					</div>
                    <span class="greytext em9 "><a href="regex-consultant.html "><b>Ask Rex</b></a></span>
					<br />
<form method="get " action="https://www.google.com/search " 
        target="_blank ">
        <input type="text "   name="q " size="14 "  maxlength="255 "
		placeholder="search the site "
        value=""
		class = "searchbox "
		/>
        <input type="hidden "  name="sitesearch "
        value="RexEgg.com " /><input type="image " src="https://d1go27vtttaqyn.cloudfront.net/search2.png "
        alt="Search " title="Search " /></form>
</div><br />

</aside>
</div><!-- Top Anchor -->
<footer>
<br /><b>&copy; Copyright RexEgg.com</b>
<br /><br /><br /><br /><br /><br /><br />
	
</footer>
<!-- POWERED BY BIG FOOT -->
</body></html>

"###;

fn main() {
    println!("Hello, world!");
    let balerin = r"(?x)
    \s*
    (?P<balise>
        <
        \s*
        (?P<end_start>/)?
        \s*
        (?P<name>[^\ >]+)
        \s*
        (?P<params>[^/>]+)?
        \s*
        (?P<end_end>/)?
        \s*
        >
        (?P<after>[^<]+)?
    )
    ";
    let balarer = r"(?x)(?s)
    \s*
    (?P<balise>
        (?P<lol>
            (?P<front>\w*)?
            </?[^<>]+>[\s\n]*
            #(?P<end>\w*)?
        ){2}
    )
    \s*
    ";
    let temp = r"(?x)(?s)
        \ *
        (?P<balise>
            <
            \ *
            (?P<name>
                [^\ />]+
            )
            (?:
                (?:
                    (?:
                        \ (?P<params>
                            [^>]+
                        )
                    )?
                    >
                    (?P<inner>
                        .+
                    )?
                    (?:
                        </
                        (?P<ender>
                            [^>]*
                        )
                        >
                    )+?
                )|
                (?:
                    \ *
                    (?P<caracs>
                        [^>]+
                    )
                    /
                    >
                )
            )
            
        )";
    for cap in Regex::new(
        balerin,
        // r"(?P<balise><(?P<balise_name>[^ >]+)(?:(?: (?P<params>[^>]+))?>(?P<inner>[^<]+)?</.*>)|(?: (?P<aparams>[^>]+))?>)",
        // r"(?P<alone_balise><(?P<balise_name>[^ >]+)(?: (?P<params>[^>]+))?>)",
        // r"(?P<alone_balise><[^>]*>)|(?P<balise><(?P<balise_name>[^ ]*)(?: [^>]*)?>[^<]*</\balise_name>)",
    )
    .unwrap()
    .captures_iter(EXAMPLE)
    {
        let temp = match cap.name("name") {
            Some(value) => value.as_str(),
            None => "",
        };
        println!("{:#?}{:#?}", cap, temp);
    }

    // for cap in Regex::new(FIND_SVG).unwrap().captures_iter(TEXT1) {
    //     println!("{:#?}", cap);
    //     for cape in Regex::new(VALID_COMMAND_CONST)
    //         .unwrap()
    //         .captures_iter(cap.get(1).unwrap().into())
    //     {
    //         println!("{:#?}", cape);
    //     }
    // }
    // let shape1 = "m 150,150 a 25,25 0 1,1 50,0 a 25,25 0 1,1 -50,0 z";
    // let shape2 = "m 40 254 s 35 -27 30 -69 s 33 -49 75 -25 z";
    // let shape3 = "m 40. 254.5  z";
    // let wrong_shape = "m l 250 a -400, -350 .";
    // let correct_complex_path = "M10,10 l 5,7 C-5,7.2,.3-16,24,10  z";
    // let _tests = vec![
    //     "",
    //     "M-11.11,-22 L.33-44  ac55    66 h77  M88 .99  Z",
    //     "M500,500 L500,200 L800,500 z M400,600 L400,900 L100,600 z",
    //     "M70.491,50.826c-2.232,1.152-6.913,2.304-12.817,2.304c-13.682,0-23.906-8.641-23.906-24.626c0-15.266,10.297-25.49,25.346-25.49c5.977,0,9.865,1.296,11.521,2.16l-1.584,5.112C66.747,9.134,63.363,8.27,59.33,8.27c-11.377,0-18.938,7.272-18.938,20.018c0,11.953,6.841,19.514,18.578,19.514c3.888,0,7.777-0.792,10.297-2.016L70.491,50.826z",
    //     "M10,10",
    //     "Z"
    // ];

    // let all = vec![
    //     "",
    //     "M-11.11,-22 L.33-44  ac55    66 h77  M88 .99  Z",
    //     "M500,500 L500,200 L800,500 z M400,600 L400,900 L100,600 z",
    //     "M70.491,50.826c-2.232,1.152-6.913,2.304-12.817,2.304c-13.682,0-23.906-8.641-23.906-24.626c0-15.266,10.297-25.49,25.346-25.49c5.977,0,9.865,1.296,11.521,2.16l-1.584,5.112C66.747,9.134,63.363,8.27,59.33,8.27c-11.377,0-18.938,7.272-18.938,20.018c0,11.953,6.841,19.514,18.578,19.514c3.888,0,7.777-0.792,10.297-2.016L70.491,50.826z",
    //     "M10,10",
    //     "Z",
    //     "m 150,150 a 25,25 0 1,1 50,0 a 25,25 0 1,1 -50,0 z",
    //     "m 40 254 s 35 -27 30 -69 s 33 -49 75 -25 z",
    //     "m 40. 254.5  z",
    //     "m l 250 a -400, -350 .",
    //     "M10,10 l 5,7 C-5,7.2,.3-16,24,10  z"
    // ];
    // for a in all {
    //     println!("");
    //     println!("{a:?}");
    //     println!("{:?}", list_ops(a));
    //     for e in list_ops(a) {
    //         println!("{}", e);
    //     }
    //     println!("{:?}", prettier_ops(a));
    // }

    // carac(shape1);
    // carac(shape2);
    // carac(shape3);
    // carac(wrong_shape);
    // carac(correct_complex_path);
    // carac("C-5,7.2,.3-16,24,10");
    // println!("");
    // for e in tests {
    //     carac(e);
    // }

    // pub const CURVE_TO2: &str = r"(?x)
    // \ *
    // (?:
    //     [Cc]
    //     [,\s]*
    //     (?P<x1>
    //         -?[0-9]*\.?\d+
    //     )
    //     [,\s]*
    //     (?P<y1>
    //         -?[0-9]*\.?\d+
    //     )
    //     [,\s]*
    //     (?P<x2>
    //         -?[0-9]*\.?\d+
    //     )
    //     [,\s]*
    //     (?P<y2>
    //         -?[0-9]*\.?\d+
    //     )
    //     [,\s]*
    //     (?P<x>
    //         -?[0-9]*\.?\d+
    //     )
    //     [,\s]*
    //     (?P<y>
    //         -?[0-9]*\.?\d+
    //     )
    // )+
    // \ *";

    // println!(
    //     "{:#?}",
    //     Regex::new(CURVE_TO2)
    //         .unwrap()
    //         .is_match("C-5,7.2,.3-16,24,10")
    // );
    // println!("");

    // let mine = Regex::new(SVG_PATH_OPS).unwrap();
    // println!("{:#?}", mine.is_match(correct_complex_path));
    // for cap in mine.captures_iter(correct_complex_path) {
    //     println!("{:?}", matcher(&cap));
    //     println!("{:?}", cap);
    //     println!("{:?}", cap.get(1));
    //     println!("");
    //     let mut azd = cap.iter();
    //     azd.next(); // we skip the first item which is the one which doesn't mater
    //     while let Some(to_print) = azd.next() {
    //         println!("{:?}", to_print);
    //     }
    //     println!("");
    // }

    // let temp = Regex::new(IS_VALID_DESCRIPTOR).unwrap();
    // let mine_old = Regex::new(r"( \d{1,} \d{1,} )").unwrap();
    // let mine = Regex::new(
    //     r"(?x)
    //   (?P<x>\d{1,}.?\d{1,}) # x pos
    //   [\ ,]
    //   (?P<y>\d{1,}.?\d{1,}) # y pos
    // ",
    // )
    // .unwrap();
    // let regex_num_point_raw = Regex::new(
    //     r"(?x)
    // (?P<num> # pos
    //     (?P<unit>\d{1,})
    //     (?:.?(?P<deci>\d+)?)
    // )
    // ",
    // )
    // .unwrap();

    // println!("{:#?}", temp.is_match(shape1));
    // println!("{:#?}", temp.is_match(shape2));
    // println!("{:#?}", temp.is_match(wrong_shape));
    // println!("");
    // println!("{:#?}", mine.is_match(shape1));
    // println!("{:#?}", mine.is_match(shape2));
    // println!("{:#?}", mine.is_match(wrong_shape));
    // println!("");
    // for cap in mine.captures_iter(shape1) {
    //     println!("{:?}", cap);
    // }
    // println!("");
    // for cap in mine.captures_iter(shape2) {
    //     println!("{:?}", cap);
    // }
    // println!("");
    // for cap in mine.captures_iter(shape3) {
    //     println!("{:?}", cap);
    // }
    // println!("");
    // println!("");
    // println!("");
    // IS_VALID_DESCRIPTOR.test( shape2 );
    //    -> true
    // IS_VALID_DESCRIPTOR.test( wrong_shape );
    // //    -> false
    // shape1.match( VALID_COMMAND_CONST );
    // //    -> [ 'm 150,150', 'a 25,25 0 1,1 50,0', 'a 25,25 0 1,1 -50,0', 'z' ]
    // shape2.match( VALID_COMMAND_CONST ).map( command => command.split( r"/[\s,]/" ).map( parameter => parseInt( parameter ) || parameter ) );
    // //    -> [ [ 'm', 40, 254 ], [ 's', 35, -27, 30, -69 ], [ 's', 33, -49, 75, -25 ], [ 'z' ] ]
}

#[test]
fn testing_svg_float_point() {
    let regex_num = Regex::new(REGEX_SVG_NUM_POINT).unwrap();

    let int = "254";
    assert!(regex_num.is_match(int));
    for cap in regex_num.captures_iter(int) {
        assert_eq!(
            format!("{:?}", cap),
            String::from(
                r#"Captures({0: Some("254"), "num": Some("254"), "unit": Some("254"), "deci": None})"#
            )
        );
    }

    let float = "2.54";
    assert!(regex_num.is_match(float));
    for cap in regex_num.captures_iter(float) {
        assert_eq!(
            format!("{:?}", cap),
            String::from(
                r#"Captures({0: Some("2.54"), "num": Some("2.54"), "unit": Some("2"), "deci": Some("54")})"#
            )
        );
    }

    let float_no_deci = "2.";
    assert!(regex_num.is_match(float_no_deci));
    for cap in regex_num.captures_iter(float_no_deci) {
        assert_eq!(
            format!("{:?}", cap),
            String::from(
                r#"Captures({0: Some("2."), "num": Some("2."), "unit": Some("2"), "deci": None})"#
            )
        );
    }

    let float_no_unit = ".54";
    assert!(regex_num.is_match(float_no_unit));
    for cap in regex_num.captures_iter(float_no_unit) {
        assert_eq!(
            format!("{:?}", cap),
            String::from(
                r#"Captures({0: Some(".54"), "num": Some(".54"), "unit": None, "deci": Some("54")})"#
            )
        );
    }
}

#[test]
fn testing_svg_float_coma() {
    let regex_num = Regex::new(consts::REGEX_SVG_NUM_COMA).unwrap();
    let int = "254";
    assert!(regex_num.is_match(int));
    for cap in regex_num.captures_iter(int) {
        assert_eq!(
            format!("{:?}", cap),
            String::from(
                r#"Captures({0: Some("254"), "num": Some("254"), "unit": Some("254"), "deci": None})"#
            )
        );
    }

    let float = "2,54";
    assert!(regex_num.is_match(float));
    for cap in regex_num.captures_iter(float) {
        assert_eq!(
            format!("{:?}", cap),
            String::from(
                r#"Captures({0: Some("2,54"), "num": Some("2,54"), "unit": Some("2"), "deci": Some("54")})"#
            )
        );
    }

    let float_no_deci = "2,";
    assert!(regex_num.is_match(float_no_deci));
    for cap in regex_num.captures_iter(float_no_deci) {
        assert_eq!(
            format!("{:?}", cap),
            String::from(
                r#"Captures({0: Some("2,"), "num": Some("2,"), "unit": Some("2"), "deci": None})"#
            )
        );
    }

    let float_no_unit = ",54";
    assert!(regex_num.is_match(float_no_unit));
    for cap in regex_num.captures_iter(float_no_unit) {
        assert_eq!(
            format!("{:?}", cap),
            String::from(
                r#"Captures({0: Some(",54"), "num": Some(",54"), "unit": None, "deci": Some("54")})"#
            )
        );
    }
}
