use std::{borrow::BorrowMut, default, fmt::Display};

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
const EXAMPLE: &str = include_str!("test2.html");

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
enum Balise {
    Groupe {
        name: String,
        params_in: Vec<String>,
        params_out: Vec<String>,
        inners: Vec<Balise>,
        after_in: String,
        after_out: String,
    },
    Solo {
        name: String,
        params: Vec<String>,
        after: String,
    },
}

impl Default for Balise {
    fn default() -> Self {
        Balise::Solo {
            name: String::new(),
            params: vec![],
            after: String::new(),
        }
    }
}

impl Display for Balise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Balise::Groupe {
                name,
                params_in,
                params_out,
                inners,
                after_in,
                after_out,
            } => {
                let atze = {
                    let mut to_print = String::from("");
                    for inner in inners.iter() {
                        to_print.push_str(&format!("{}", inner))
                    }
                    to_print
                };
                let mut marker = String::from("");
                if !name.contains("--") && params.len() != 0 {
                    marker = String::from(" ")
                }
                write!(f, "{atze}</{name}{marker}{}>{after}", params.join(" "))
            }
            Balise::Solo {
                name,
                params,
                after,
            } => {
                let mut marker = String::from("");
                if !name.ends_with("--") || (*name == String::from("!--")) {
                    marker = String::from(" ")
                }
                write!(f, "<{name}{marker}{}>{after}", params.join(" "))
            }
        }
    }
}

impl Balise {
    pub fn name(&self) -> String {
        match self {
            Balise::Groupe { name, .. } => name.to_string(),
            Balise::Solo { name, .. } => name.to_string(),
        }
    }
    pub fn to_group(&self) -> Balise {
        match self {
            Balise::Groupe { .. } => self.clone(),
            Balise::Solo {
                name,
                params,
                after,
            } => Balise::Groupe {
                name: name.to_string(),
                params_in: params.to_vec(),
                params_out: vec![],
                inners: vec![],
                after_in: after.to_string(),
                after_out: String::new(),
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
    let balerin = r"(?x)
    \s*
    (?P<balise>
        <
        (?:
            \s*
            (?P<end_start>/)?
            \s*
            (?P<name>[^\s>]+)
            \s*
            (?P<params>[^>]+)?
            \s*
            (?P<end_end>/)?
            \s*
            >
            (?P<after>[^<]+)?
        )
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

    // collect alla balise with a name into vectaze
    let mut vectaze = vec![];
    for cap in Regex::new(
        balerin,
        // r"(?P<balise><(?P<balise_name>[^ >]+)(?:(?: (?P<params>[^>]+))?>(?P<inner>[^<]+)?</.*>)|(?: (?P<aparams>[^>]+))?>)",
        // r"(?P<alone_balise><(?P<balise_name>[^ >]+)(?: (?P<params>[^>]+))?>)",
        // r"(?P<alone_balise><[^>]*>)|(?P<balise><(?P<balise_name>[^ ]*)(?: [^>]*)?>[^<]*</\balise_name>)",
    )
    .unwrap()
    .captures_iter(EXAMPLE)
    {
        if let Some(name) = cap.name("name") {
            println!("{:?}\n{:?}\n", name.as_str(), cap);
            vectaze.push(cap);
        }
    }

    //create the balise my_html to be the helder for the whole struct
    let mut my_html = Balise::Groupe {
        name: String::from("heml"),
        params: vec![],
        inners: vec![],
        after: String::from(""),
    };

    // we accumultate the groups in a vec and fuse the last list of balise into the last element of the element before to (wrapping them)

    // we collect all possible balises and put them in a vec, assuring groups are Balise::Groupe
    let mut aze = vec![];
    let mut index = 0;
    let mut name_pool = vec![];
    let mut _index_pool = vec![];
    let mut depth = 0_isize;
    for e in vectaze.iter().rev() {
        index += 1;

        let current_balise = Balise::Solo {
            name: e.name("name").unwrap().as_str().to_string(),
            params: match e.name("params") {
                Some(params) => vec![params.as_str().to_string()],
                None => vec![],
            },
            after: match e.name("after") {
                Some(after) => String::from(after.as_str()),
                None => String::from(""),
            },
        };

        let ender = e.name("end_start").is_some();
        if ender {
            // if we found a group ender that means that we are going deeper
            aze.push((depth, current_balise.name()));
            name_pool.push((depth, current_balise.name()));
            _index_pool.push(current_balise.to_group());
            depth += 1;
        } else {
            // if we found the opening of the group
            if name_pool.contains(&(depth - 1, current_balise.name())) {
                // then we are closing the group
                let pos_to_pop = name_pool
                    .binary_search(&(depth - 1, current_balise.name()))
                    .unwrap();
                name_pool.remove(pos_to_pop);
                depth -= 1;
            }
            aze.push((depth, current_balise.name()));
            _index_pool.push(current_balise);
        }
        // println!("{:?}\n{:#?}\n", name_pool, aze.iter().rev());
        // std::thread::sleep(std::time::Duration::from_millis(1000 as u64));
    }

    // print balise depth with tabs and number and the name of the tab
    // for e in aze.iter().rev() {
    //     println!(
    //         "{}{}{:?}",
    //         String::from(" ").repeat(e.0.abs() as usize),
    //         e.0,
    //         e.1
    //     );
    // }
    // for e in name_pool.iter().zip(_index_pool) {
    //     println!("{:?}{:?}", e.0, e.1);
    // }

    // fuse same depth balises to a vec of balises, to allow easy management of them
    let mut to_print = vec![];
    let mut lastes_depth = 0;
    let mut temp_string = String::new();
    let mut temp_vec = vec![];
    let mut to_vec = vec![];
    for e in aze.iter().zip(_index_pool) {
        if lastes_depth == e.0 .0 {
            temp_vec.push(e.1);
            temp_string.push_str(&e.0 .1);
            temp_string.push_str(" ");
        } else {
            to_vec.push(temp_vec.clone());
            to_print.push((lastes_depth, temp_string));

            lastes_depth = e.0 .0;
            temp_vec.clear();
            temp_string = String::new();

            temp_vec.push(e.1);
            temp_string.push_str(&e.0 .1);
            temp_string.push_str(" ");
        }
    }
    to_vec.push(temp_vec.clone());
    to_print.push((lastes_depth, temp_string));
    println!("{to_print:?}\n\n{to_vec:?}");
    // use the fused vec to fold the groups
    let mut fuck_you: Vec<Vec<Balise>> = vec![];
    let mut las_depth = -1;
    for e in to_vec.iter_mut().zip(&to_print).rev() {
        let real: Vec<&Balise> = e.0.iter().rev().collect();

        // println!("(({:?}, {:?}), {:?})", e.1 .0, e.1 .1, real);
        let current_depth = e.1 .0;
        // println!("fuck you {:?}: {:?}\n\n", fuck_you.len(), fuck_you);
        if current_depth > las_depth {
            //going deeper so adding an element to the vec
            e.0.reverse();
            fuck_you.push(e.0.to_vec())
        }
        if current_depth < las_depth {
            //getting out of the depth -> meaning that we are grouping things -> removing for the vec
            // println!("\nfuckazeazrertze you: {:?}\n\n", e.0.last_mut());
            match e.0.last_mut() {
                Some(lol) => match lol {
                    Balise::Groupe { inners, .. } => *inners = fuck_you.pop().unwrap(),
                    Balise::Solo { .. } => (),
                },
                None => unreachable!(),
            }
            let frgze = fuck_you.len() - 1;
            e.0.reverse();
            fuck_you[frgze].append(&mut e.0.to_vec())
        }
        if las_depth == current_depth {
            //we are cool so we add to the depth in the vec (the last element)
            let frgze = fuck_you.len() - 1;
            e.0.reverse();
            fuck_you[frgze].append(&mut e.0.to_vec())
        }
        las_depth = current_depth;
    }
    // // println!("{fuck_you:#?}");
    let final_html = &fuck_you[0];

    let mut to_file = String::from("");
    // // here the group balises have an issue as there is the starter balise and the group balise
    // let mut last_element: Balise = Default::default();
    // for e in final_html {
    //    //  match e {
    //         Balise::Groupe {
    //             name,
    //             params,
    //             inners,
    //             after,
    //         } => if last_element.name() == name {},
    //         Balise::Solo { .. } => (),
    //     }
    //     last_element = *e;
    // }

    // for e in final_html {
    //     println!("{e}");
    to_file += &format!("{e}");
    // }

    {
        let path = std::path::Path::new("src/result2.html");
        let display = path.display();
        let mut file = match std::fs::File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        match std::io::Write::write_all(&mut file, to_file.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    // for e in to_vec.iter_mut().zip(to_print).rev() {
    //     let real: Vec<&Balise> = e.0.iter().rev().collect();

    //     println!("(({:?}, {:?}), {:?})\n", e.1 .0, e.1 .1, real);
    // }
    // for e in to_vec.iter().rev() {
    //     println!("{:?}", e);
    // }

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
