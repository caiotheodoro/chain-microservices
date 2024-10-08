from proto import ReportGRPC
import logging
from grpc import StatusCode
from google.protobuf.empty_pb2 import Empty
from app.models import ReportDBInterface

class MyReportService(ReportGRPC.ReportService):
    def __init__(self, reportDB: ReportDBInterface):
        self.reportDB = reportDB
        super().__init__()
    
    def ListReports(self,request, context):
        logging.info(f"MyReportService.ListReports: emessage recieved...{request}")
        page = request.page
        size = request.size
        try:
            result = self.reportDB.list(page=page,size=size)
            return result.toProto()
        except Exception as e:
            logging.error(f"MyReportService.ListReports: error occured...{e}")
            context.set_code(StatusCode.INTERNAL)
            context.set_details("An internal server error occurred.")
            return Empty()

    async def CreateReport(self, request, context):
        logging.info(f"MyReportService.CreateReport: message received...{request}")
        try:
            report = Report(
                Topic=request.topic,
                CreatedTime=datetime.now(),
                ReportData=request.report_data
            )
            self.reportDB.create(report)
            return ReportGRPCTypes.CreateReportResponse(success=True)
        except Exception as e:
            logging.error(f"MyReportService.CreateReport: error occurred...{e}")
            context.set_code(StatusCode.INTERNAL)
            context.set_details("An internal server error occurred.")
            return ReportGRPCTypes.CreateReportResponse(success=False)