/*
 * Copyright (C) 2023 Muqiu Han
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */

use crate::cli::CommandLineParser;
use crate::commands::cat_file::CatFile;
use crate::commands::hash_object::HashObject;
use crate::commands::init::Init;

pub mod cat_file;
mod hash_object;
pub mod init;

pub fn command(args: CommandLineParser) {
    match args {
        CommandLineParser::Init { force, path } => {
            Init { force, path }.init();
        }

        CommandLineParser::CatFile { typ, object } => CatFile { typ, object }.cat(),

        CommandLineParser::HashObject { typ, write, path } => {
            HashObject { typ, write, path }.hash_object();
        }
    }
}
