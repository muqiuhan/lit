module vercos.Argu

open Argu

[<RequireQualifiedAccess>]
type CliArgs =
    | [<MainCommand; Last>] Args of string list

    interface IArgParserTemplate with
        member this.Usage =
            match this with
            | Args(_) -> "__VERCOS_ARGS__"

[<RequireQualifiedAccess>]
type CLIArguments =
    | [<CliPrefix(CliPrefix.None)>] Version
    | [<CliPrefix(CliPrefix.None)>] Help
    | [<CliPrefix(CliPrefix.None)>] Add
    | [<CliPrefix(CliPrefix.None)>] Init
    | [<CliPrefix(CliPrefix.None)>] Log
    | [<CliPrefix(CliPrefix.None)>] Rm
    | [<CliPrefix(CliPrefix.None)>] Tagging
    | [<CliPrefix(CliPrefix.None)>] Status
    | [<CliPrefix(CliPrefix.None)>] Cat_File
    | [<CliPrefix(CliPrefix.None)>] Check_Ignore
    | [<CliPrefix(CliPrefix.None)>] Checkout
    | [<CliPrefix(CliPrefix.None)>] Commit //  ParseResults<CleanArgs>
    | [<CliPrefix(CliPrefix.None)>] Hash_Object
    | [<CliPrefix(CliPrefix.None)>] Ls_Files
    | [<CliPrefix(CliPrefix.None)>] Ls_Tree
    | [<CliPrefix(CliPrefix.None)>] Rev_Parse
    | [<CliPrefix(CliPrefix.None)>] Show_Ref


    interface IArgParserTemplate with
        member s.Usage =
            match s with
            | Version -> "Display version information about vercos"
            | Add -> "Add file contents to the index"
            | Init -> "Create an empty vercos repository or reinitialize an existing one"
            | Log -> "Show commit logs"
            | Rm -> "Remove files from the working tree and from the index"
            | Tagging -> "Create, list, delete or verify a tag object signed with GPG"
            | Status -> "Show the working tree status"
            | Cat_File -> "Provide content or type and size information for repository objects"
            | Check_Ignore -> "Debug gitignore / exclude files"
            | Checkout -> "Switch branches or restore working tree files"
            | Commit -> "Record changes to the repository"
            | Hash_Object -> "Compute object ID and optionally create an object from a file"
            | Ls_Files -> "Show information about files in the index and the working tree"
            | Ls_Tree -> "List the contents of a tree object"
            | Rev_Parse -> "Pick out and massage parameters"
            | Help -> "Display help information about vercos"
            | Show_Ref -> "List references in a local repository"

type Parser =
    static member private ErrorHandler =
        { new IExiter with
            member this.Exit(msg: string, errorCode: ErrorCode) =
                match errorCode with
                | ErrorCode.HelpText -> exit 0
                | _ -> exit 0

            member this.Name: string = "_" }

    static member private parser =
        ArgumentParser.Create<CLIArguments>(programName = "vercos", errorHandler = Parser.ErrorHandler)

    static member public Parse(argv) = Parser.parser.Parse(argv)

    static member public Usage() = Parser.parser.PrintUsage()