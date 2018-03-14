; ModuleID = 'greet'
source_filename = "greet"

%opaque = type opaque

@0 = private unnamed_addr constant [2 x i8] c"w\00"
@1 = private unnamed_addr constant [2 x i8] c"r\00"
@2 = private unnamed_addr constant [20 x i8] c"what is your name?\0A\00"
@3 = private unnamed_addr constant [7 x i8] c"Hello \00"

declare %opaque* @fdopen(i32, i8*)

declare i32 @fputs(i8*, %opaque*)

declare i8* @fgets(i8*, i32, %opaque*)

declare i8* @malloc(i32)

define %opaque* @stdout() {
entry:
  %0 = call %opaque* @fdopen(i32 1, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @0, i32 0, i32 0))
  ret %opaque* %0
}

define %opaque* @stdin() {
entry:
  %0 = call %opaque* @fdopen(i32 0, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @1, i32 0, i32 0))
  ret %opaque* %0
}

define i8* @read() {
entry:
  %0 = call i8* @malloc(i32 200)
  %1 = call %opaque* @stdin()
  %2 = call i8* @fgets(i8* %0, i32 200, %opaque* %1)
  ret i8* %2
}

define void @write(i8*, %opaque*) {
entry:
  %2 = call i32 @fputs(i8* %0, %opaque* %1)
  ret void
}

define i32 @main() {
entry:
  %0 = call %opaque* @stdout()
  call void @write(i8* getelementptr inbounds ([20 x i8], [20 x i8]* @2, i32 0, i32 0), %opaque* %0)
  %1 = call i8* @read()
  call void @write(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @3, i32 0, i32 0), %opaque* %0)
  call void @write(i8* %1, %opaque* %0)
  ret i32 0
}
