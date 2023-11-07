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

pub mod cat_file;
pub mod init;

pub fn command(args: &CommandLineParser) {
    match args {
        CommandLineParser::Init { path, force } => {
            info!("create lit repository on {}...", path);
            init::Init::create(path.clone(), *force);
            info!("create ok!");
        }

        CommandLineParser::CatFile { typ, object } => {
            info!("get the content of repository objects `{}`", object);
            (cat_file::CatFile {
                typ: typ.clone(),
                object: object.clone(),
            })
            .cat()
        }
    }
}
